; The Windows installer, compiled by tools/package-windows.sh (ISCC /D defines: AppVersion,
; AppSourceDir, IconFile, OutputDir, OutputBaseName, Arch; when signing, SignTool: the name of an
; ISCC /S sign tool). This fork has a separate installer and Windows app identity from YMusicV4.
;
; Build input: AppSourceDir holds AUTUMN.exe, ffmpeg.exe and the libav / MinGW runtime DLLs.

#define AppName "AUTUMN"
#define AppExeName "AUTUMN.exe"
#define AppPublisher "AUTUMN"
#define AppURL "https://ymusic.io"
#define MediaProgId "AUTUMN.Media"
; The taskbar and the media flyout find the app's name and icon through the shortcut carrying it,
; and a pin merges with the running window only when they match (crates/ymusic desktop.rs).
#define AppUserModelId "io.autumn.windows"
#define AppId "{{A6E59CB3-2B4A-4DF3-9B20-3B8A58C1D2F7}"

[Setup]
AppId={#AppId}
AppName={#AppName}
AppVersion={#AppVersion}
AppVerName={#AppName} {#AppVersion}
AppComments=YOUR MUSIC. YOUR MOMENT.
AppPublisher={#AppPublisher}
AppPublisherURL={#AppURL}
AppSupportURL={#AppURL}
AppUpdatesURL={#AppURL}
DefaultDirName={autopf}\{#AppName}
DefaultGroupName={#AppName}
DisableProgramGroupPage=auto
DisableDirPage=auto
AllowNoIcons=yes
OutputDir={#OutputDir}
OutputBaseFilename={#OutputBaseName}
SetupIconFile={#IconFile}
UninstallDisplayIcon={app}\{#AppExeName}
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
; Per-machine, as the JVM app: an upgrade lands in its folder.
PrivilegesRequired=admin
ArchitecturesAllowed={#Arch}
ArchitecturesInstallIn64BitMode={#Arch}
MinVersion=10.0
CloseApplications=force
ChangesAssociations=yes
RestartApplications=no
#ifdef SignTool
SignTool={#SignTool}
SignedUninstaller=yes
; A failed sign may have waited out the phone prompt; a retry would wait again.
SignToolRetryCount=0
#endif

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

; The JVM app's jlinked runtime and jars; an upgrade from it must not leave them behind.
[InstallDelete]
Type: filesandordirs; Name: "{app}\app"
Type: filesandordirs; Name: "{app}\runtime"

[Files]
Source: "{#AppSourceDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Registry]
; ymusic:// links reach the running app through the single instance.
Root: HKA; Subkey: "Software\Classes\ymusic"; ValueType: string; ValueData: "URL:AUTUMN"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\ymusic"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKA; Subkey: "Software\Classes\ymusic\DefaultIcon"; ValueType: string; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\ymusic\shell\open\command"; ValueType: string; ValueData: """{app}\{#AppExeName}"" ""%1"""
; Lists the app under Open With without taking any type's default.
Root: HKA; Subkey: "Software\Classes\{#MediaProgId}"; ValueType: string; ValueData: "Media file"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#MediaProgId}\DefaultIcon"; ValueType: string; ValueData: "{app}\{#AppExeName},0"
Root: HKA; Subkey: "Software\Classes\{#MediaProgId}\shell\open\command"; ValueType: string; ValueData: """{app}\{#AppExeName}"" ""%1"""
Root: HKA; Subkey: "Software\Classes\.mp3\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.m4a\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.m4b\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.aac\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.flac\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.wav\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.ogg\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.oga\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.opus\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.mka\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.mp4\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.m4v\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.mov\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.webm\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\.mkv\OpenWithProgids"; ValueType: string; ValueName: "{#MediaProgId}"; ValueData: ""; Flags: uninsdeletevalue

[Icons]
Name: "{group}\{#AppName}"; Filename: "{app}\{#AppExeName}"; AppUserModelID: "{#AppUserModelId}"
Name: "{autodesktop}\{#AppName}"; Filename: "{app}\{#AppExeName}"; AppUserModelID: "{#AppUserModelId}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#AppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(AppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
