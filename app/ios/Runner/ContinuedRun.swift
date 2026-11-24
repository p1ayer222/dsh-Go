import UIKit

/// iOS 26+ background continued processing is not compiled into this branch (SDK 16 / deployment 16).
/// The engine pauses in background as on older iOS. Main thread only.
final class ContinuedRun {
  var onChange: ((Bool) -> Void)?

  func update(running: Bool) {
    _ = running
  }
}
