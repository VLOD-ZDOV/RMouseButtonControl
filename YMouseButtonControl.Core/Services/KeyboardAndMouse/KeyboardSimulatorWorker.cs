using System;

namespace YMouseButtonControl.Core.Services.KeyboardAndMouse;

public class KeyboardSimulatorWorker : IDisposable
{
    public void Run()
    {
        // No-op: C# simulation engine disabled. Rust daemon handles all macros and input simulation.
    }

    public void Dispose()
    {
    }
}
