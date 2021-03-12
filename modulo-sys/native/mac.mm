#import <AppKit/AppKit.h>

void ActivateApp() {
    [[NSRunningApplication currentApplication] activateWithOptions:(NSApplicationActivateAllWindows | NSApplicationActivateIgnoringOtherApps)];
}

void SetWindowStyles(NSWindow * window) {
    window.titleVisibility = NSWindowTitleHidden;
	window.styleMask &= ~NSWindowStyleMaskTitled;
    window.movableByWindowBackground = true;
}