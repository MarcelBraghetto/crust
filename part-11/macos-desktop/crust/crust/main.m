#import <Cocoa/Cocoa.h>

int main(int argc, const char * argv[]) {
    NSTask *task = [NSTask new];
    task.launchPath = [[NSBundle mainBundle] pathForResource:@"crust" ofType:nil];
    [task launch];
    [task waitUntilExit];
}
