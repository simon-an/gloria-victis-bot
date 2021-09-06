fn main() {
    windows::build! {
        Windows::ApplicationModel::*,
        Windows::Foundation::IAsyncOperation,
        Windows::Gaming::UI::*,
        Windows::Gaming::Preview::*,
        Windows::Gaming::Preview::GamesEnumeration::*,
        Windows::Win32::Graphics::DirectShow::*,
        Windows::Data::Xml::Dom::*,
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, * },
        Windows::Graphics::SizeInt32,
        Windows::Win32::System::WinRT::{
            CreateDispatcherQueueController, ICompositorDesktopInterop, RoInitialize,
        },
        Windows::UI::Colors,
        Windows::UI::Composition::Desktop::DesktopWindowTarget,
        Windows::UI::Composition::{
            CompositionColorBrush, CompositionContainerShape, CompositionEllipseGeometry,
            CompositionNineGridBrush, CompositionScopedBatch, CompositionShapeCollection,
            CompositionSpriteShape, Compositor, ShapeVisual, SpriteVisual,
            Vector3KeyFrameAnimation, VisualCollection,
        },
        // Windows::UI::{
        //     Colors, Color
        //   },
        //   Windows::UI::Xaml::{
        //     VerticalAlignment, HorizontalAlignment
        //   },
        //   Windows::UI::Xaml::Media::SolidColorBrush,
        //   Windows::UI::Xaml::Controls::{
        //     UIElementCollection, TextBlock, StackPanel, Grid, Page
        //   },
        //   Windows::UI::Xaml::Documents::*,
        //   Windows::UI::Xaml::Hosting::{
        //     WindowsXamlManager, DesktopWindowXamlSource
        //   },
        //   Windows::UI::Xaml::Markup::XamlReader,
        //   Windows::Foundation::Uri,
        //   Windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_TYPE},
        //   Windows::Win32::UI::DisplayDevices::RECT,
        //   Windows::Win32::Graphics::Gdi::{
        //     HBRUSH, UpdateWindow, BeginPaint, EndPaint, TextOutW, PAINTSTRUCT
        //   },
        //   Windows::Win32::System::SystemServices::{
        //     PWSTR, GetModuleHandleW
        //   },
        //   Windows::Win32::UI::MenusAndResources::HMENU,
        //   Windows::Win32::UI::WindowsAndMessaging::{
        //     HWND, CreateWindowExW, RegisterClassExW, DefWindowProcW, WNDCLASSEXW, CS_VREDRAW, CS_HREDRAW, LoadIconW, LoadCursorW,
        //     IDI_APPLICATION, IDC_APPSTARTING, PostQuitMessage, WS_OVERLAPPEDWINDOW, WS_VISIBLE, CW_USEDEFAULT, GetMessageW, TranslateMessage,
        //     DispatchMessageW, WM_QUIT, WM_CREATE, WS_CHILD, WS_BORDER, WM_DESTROY, WM_CREATE, SetWindowPos, SWP_SHOWWINDOW, WM_SIZE, GetClientRect,
        //     MoveWindow, ShowWindow, SW_SHOW
        //   }
    };
}
