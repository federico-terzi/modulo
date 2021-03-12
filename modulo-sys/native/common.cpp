#include "common.h"

#ifdef __WXMSW__
#include <windows.h>
#endif
#ifdef __WXOSX__
#include "mac.h"
#endif

void setFrameIcon(const char * iconPath, wxFrame * frame) {
    if (iconPath) {
        wxString iconPath(iconPath);
        wxBitmapType imgType = wxICON_DEFAULT_TYPE;

        #ifdef __WXMSW__
            imgType = wxBITMAP_TYPE_ICO;
        #endif

        wxIcon icon;
        icon.LoadFile(iconPath, imgType);
        if (icon.IsOk()) {
            frame->SetIcon(icon);
        }
    }
}

void Activate(wxFrame * frame) {
    #ifdef __WXMSW__

    HWND handle = frame->GetHandle();
    if (handle == GetForegroundWindow()) {
        return;
    }

    if (IsIconic(handle)) {
        ShowWindow(handle, 9);
    }

    INPUT ip;
    ip.type = INPUT_KEYBOARD;
    ip.ki.wScan = 0;
    ip.ki.time = 0;
    ip.ki.dwExtraInfo = 0;
    ip.ki.wVk = VK_MENU;
    ip.ki.dwFlags = 0;

    SendInput(1, &ip, sizeof(INPUT));
    ip.ki.dwFlags = KEYEVENTF_KEYUP;

    SendInput(1, &ip, sizeof(INPUT));

    SetForegroundWindow(handle);

    #endif
    #ifdef __WXOSX__
    ActivateApp();
    #endif
}

void SetupWindowStyle(wxFrame * frame) {
    #ifdef __WXOSX__
        SetWindowStyles((NSWindow*) frame->MacGetTopLevelWindowRef());
    #endif
}