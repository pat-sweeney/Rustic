#![warn(unused_imports)]
#![warn(dead_code)]
#[allow(non_snake_case)]

use windows::
{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

//**********************************************************************
// Struct: App
// App is a holder for stuff related to creating the app
//**********************************************************************
pub struct App
{
    instance : HINSTANCE,
    window_class : PCSTR,
    hwnd : windows::Win32::Foundation::HWND,
}

impl App
{
    pub fn new() -> Self
    {
        Self
	{
	    instance : windows::Win32::Foundation::HINSTANCE(0),
	    window_class : s!(""),
	    hwnd : windows::Win32::Foundation::HWND(0),
	}
    }

    fn InitializeCaustic(&self)
    {
        println!("Initializing Caustic");
    }
}

//**********************************************************************
// Method: main
// Defines the main entry point for our MeshViewer application.
//
// Returns: Standard RUST Result<>
//**********************************************************************
fn main() -> Result<()>
{
    unsafe
    {
        let mut app = App::new();
        app.instance = GetModuleHandleA(None)?;
        debug_assert!(app.instance.0 != 0);

        app.window_class = s!("window");
        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: app.instance,
            lpszClassName: app.window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        app.hwnd = CreateWindowExA(WINDOW_EX_STYLE::default(), app.window_class, s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
            None, None, app.instance, None);

        app.InitializeCaustic();
	
        ShowWindow(app.hwnd, SW_SHOW);

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into()
	{
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system"
fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
    unsafe
    {
        match message
	{
            WM_PAINT =>
	    {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY =>
	    {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
