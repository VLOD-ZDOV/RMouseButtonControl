using System;

namespace YMouseButtonControl.BackgroundTaskRunner;

public interface IBackgroundTasksRunner : IDisposable { }

public class BackgroundTasksRunner : IBackgroundTasksRunner
{
    public void Dispose()
    {
    }
}
