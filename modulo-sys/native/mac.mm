#import <AppKit/AppKit.h>

void ActivateApp() {
    [[NSRunningApplication currentApplication] activateWithOptions:(NSApplicationActivateAllWindows | NSApplicationActivateIgnoringOtherApps)];
}