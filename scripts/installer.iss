#define MyAppName "RMouseButtonControl"
#define MyAppVersion "0.29.0"
#define MyAppPublisher "VLOD-ZDOV"
#define MyAppURL "https://github.com/VLOD-ZDOV/RMouseButtonControl"
#define MyAppExeName "rmouse-gui.exe"

[Setup]
AppId={{D37D3E5B-F16B-4D78-95F8-B5944B6D873F}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
OutputDir=..\dist
OutputBaseFilename=RMouseButtonControl-Windows-Setup
SetupIconFile=..\gui\resources\mouse.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64compatible

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "startup"; Description: "Run RMouseButtonControl automatically at Windows startup"; GroupDescription: "Startup options:"

[Files]
Source: "..\bundle-win-qt\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\RMouseButtonControl Background Daemon"; Filename: "{app}\rmousebuttoncontrol.exe"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
Name: "{userstartup}\{#MyAppName}"; Filename: "{app}\rmousebuttoncontrol.exe"; Tasks: startup

[Run]
Filename: "{app}\rmousebuttoncontrol.exe"; Description: "Start RMouseButtonControl Background Daemon"; Flags: nowait postinstall skipifsilent
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent unchecked
