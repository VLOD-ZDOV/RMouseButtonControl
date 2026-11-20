using System;
using System.Collections.ObjectModel;
using System.Linq;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Threading;
using Microsoft.Extensions.Logging;
using SharpHook;
using SharpHook.Native;
using SharpHook.Reactive;
using YMouseButtonControl.Core.Services.KeyboardAndMouse.Enums;
using YMouseButtonControl.Core.Services.KeyboardAndMouse.EventArgs;
using YMouseButtonControl.Core.Services.KeyboardAndMouse.Implementations.Queries.CurrentWindow;
using YMouseButtonControl.Core.Services.KeyboardAndMouse.Implementations.Queries.Profiles;
using static YMouseButtonControl.Core.Services.KeyboardAndMouse.Implementations.Queries.Profiles.ListProfiles;
using LogLevel = Microsoft.Extensions.Logging.LogLevel;

namespace YMouseButtonControl.Core.Services.KeyboardAndMouse.Implementations;

public interface IMouseListener : IDisposable
{
    IObservable<NewMouseHookMoveEventArgs> OnMouseMovedChanged { get; }
    IObservable<NewMouseHookEventArgs> OnMousePressedChanged { get; }
    IObservable<NewMouseHookEventArgs> OnMouseReleasedChanged { get; }
    IObservable<NewMouseWheelEventArgs> OnMouseWheelChanged { get; }
    void Run();
}

/// <summary>
/// Wrapper around sharphook for listening to mouse events
/// Converts mouse events to NewMouseHookEventArgs
/// </summary>
public partial class MouseListenerService : IMouseListener
{
    private readonly ILogger<MouseListenerService> _logger;
    private readonly IReactiveGlobalHook _hook;
    private readonly IGetCurrentWindow _currentWindowService;
    private readonly IDisposable? _mouseMovedDisposable;
    private readonly IDisposable? _mousePressedDisposable;
    private readonly IDisposable? _mouseReleasedDisposable;
    private readonly IDisposable? _mouseWheelDisposable;
    private readonly Subject<NewMouseHookEventArgs> _mousePressedSubject;
    private readonly Subject<NewMouseHookEventArgs> _mouseReleasedSubject;
    private readonly Subject<NewMouseHookMoveEventArgs> _mouseMovedSubject;
    private readonly Subject<NewMouseWheelEventArgs> _mouseWheelSubject;
    private readonly ReadOnlyCollection<ProfileForMouseListener> _profiles;

    public MouseListenerService(
        ILogger<MouseListenerService> logger,
        IReactiveGlobalHook hook,
        ListProfiles.Handler listProfilesHandler,
        IGetCurrentWindow currentWindowService
    )
    {
        _logger = logger;
        _hook = hook;
        _currentWindowService = currentWindowService;
        _mousePressedSubject = new Subject<NewMouseHookEventArgs>();
        _mouseReleasedSubject = new Subject<NewMouseHookEventArgs>();
        _mouseMovedSubject = new Subject<NewMouseHookMoveEventArgs>();
        _mouseWheelSubject = new Subject<NewMouseWheelEventArgs>();
        _profiles = listProfilesHandler.Execute();

        _mouseMovedDisposable = _hook
            .MouseMoved.Sample(TimeSpan.FromMilliseconds(100))
            .Where(x => !x.IsEventSimulated)
            .Subscribe(ConvertMouseMovedEvent);
        _mousePressedDisposable = _hook
            .MousePressed.Where(x => !x.IsEventSimulated)
            .Subscribe(ConvertMousePressedEvent);
        _mouseReleasedDisposable = _hook
            .MouseReleased.Where(x => !x.IsEventSimulated)
            .Subscribe(ConvertMouseReleasedEvent);
        _mouseWheelDisposable = _hook
            .MouseWheel.Where(x => !x.IsEventSimulated)
            .Subscribe(ConvertMouseWheelEvent);
    }

    public IObservable<NewMouseHookEventArgs> OnMousePressedChanged =>
        _mousePressedSubject.AsObservable();
    public IObservable<NewMouseHookEventArgs> OnMouseReleasedChanged =>
        _mouseReleasedSubject.AsObservable();
    public IObservable<NewMouseHookMoveEventArgs> OnMouseMovedChanged =>
        _mouseMovedSubject.AsObservable();
    public IObservable<NewMouseWheelEventArgs> OnMouseWheelChanged =>
        _mouseWheelSubject.AsObservable();

    public void Run()
    {
        // No-op: C# global hook disabled. Rust daemon handles all low-level OS hooks.
    }

    /// <summary>
    /// Whether to suppress the original mouse event from propagating
    /// </summary>
    /// <param name="args"></param>
    /// <returns></returns>
    private bool ShouldSuppressEvent(NewMouseHookEventArgs args) => false;

    private void ConvertMouseWheelEvent(MouseWheelHookEventArgs e)
    {
        if (e is null)
        {
            return;
        }
        switch (e.Data.Direction)
        {
            case MouseWheelScrollDirection.Vertical when e.Data.Rotation > 0:
                _mouseWheelSubject.OnNext(
                    new NewMouseWheelEventArgs(WheelScrollDirection.VerticalUp)
                );
                break;
            case MouseWheelScrollDirection.Vertical when e.Data.Rotation < 0:
                _mouseWheelSubject.OnNext(
                    new NewMouseWheelEventArgs(WheelScrollDirection.VerticalDown)
                );
                break;
            case MouseWheelScrollDirection.Vertical:
                throw new ArgumentOutOfRangeException($"{e.Data.Direction}\t{e.Data.Rotation}");
            case MouseWheelScrollDirection.Horizontal when e.Data.Rotation > 0:
                _mouseWheelSubject.OnNext(
                    new NewMouseWheelEventArgs(WheelScrollDirection.HorizontalRight)
                );
                break;
            case MouseWheelScrollDirection.Horizontal when e.Data.Rotation < 0:
                _mouseWheelSubject.OnNext(
                    new NewMouseWheelEventArgs(WheelScrollDirection.HorizontalLeft)
                );
                break;
            case MouseWheelScrollDirection.Horizontal:
                throw new ArgumentOutOfRangeException($"{e.Data.Direction}\t{e.Data.Rotation}");
            default:
                throw new ArgumentOutOfRangeException($"{e.Data.Direction}\t{e.Data.Rotation}");
        }
    }

    private void ConvertMouseReleasedEvent(MouseHookEventArgs e)
    {
        if (e is null)
        {
            return;
        }

        LogTranslateRelease(_logger, e.Data.Button);
        LogActiveWindow(_logger, _currentWindowService.ForegroundWindow);
        var args = new NewMouseHookEventArgs(
            (YMouseButton)e.Data.Button,
            e.Data.X,
            e.Data.Y,
            _currentWindowService.ForegroundWindow
        );
        if (ShouldSuppressEvent(args))
        {
            LogSuppressingButtonRelease(_logger, e.Data.Button);
            e.SuppressEvent = true;
        }
        else
        {
            LogNotSuppressingButtonRelease(_logger, e.Data.Button);
        }
        _mouseReleasedSubject.OnNext(args);
    }

    private void ConvertMousePressedEvent(MouseHookEventArgs e)
    {
        if (e is null)
        {
            return;
        }
        LogTranslateButton(_logger, e.Data.Button);
        LogActiveWindow(_logger, _currentWindowService.ForegroundWindow);

        var args = new NewMouseHookEventArgs(
            (YMouseButton)e.Data.Button,
            e.Data.X,
            e.Data.Y,
            _currentWindowService.ForegroundWindow
        );
        if (ShouldSuppressEvent(args))
        {
            LogSuppressingButtonRelease(_logger, e.Data.Button);
            e.SuppressEvent = true;
        }
        else
        {
            LogNotSuppressingButtonRelease(_logger, e.Data.Button);
        }
        _mousePressedSubject.OnNext(args);
    }

    private void ConvertMouseMovedEvent(MouseHookEventArgs e)
    {
        if (e is null)
        {
            return;
        }
        _mouseMovedSubject.OnNext(new NewMouseHookMoveEventArgs(e.Data.X, e.Data.Y));
    }

    public void Dispose()
    {
        _mouseMovedDisposable?.Dispose();
        _mousePressedDisposable?.Dispose();
        _mouseReleasedDisposable?.Dispose();
        _mouseWheelDisposable?.Dispose();

        _hook.Dispose();
    }

    [LoggerMessage(LogLevel.Information, "Translate press {Button}")]
    private static partial void LogTranslateButton(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "Suppressing {Button}: Press")]
    private static partial void LogSuppressingButtonPress(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "Not suppressing {Button}: Press")]
    private static partial void LogNotSuppressingButtonPress(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "Not suppressing {Button}: Release")]
    public static partial void LogNotSuppressingButtonRelease(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "Suppressing {Button}: Release")]
    public static partial void LogSuppressingButtonRelease(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "ACTIVE WINDOW {Foreground}")]
    private static partial void LogActiveWindow(ILogger logger, string foreground);

    [LoggerMessage(LogLevel.Information, "Translate release {Button}")]
    private static partial void LogTranslateRelease(ILogger logger, MouseButton button);

    [LoggerMessage(LogLevel.Information, "Starting mouse listener")]
    private static partial void LogStartup(ILogger logger);
}
