## TODO
#### Observer

- TwoTeamParameters + RobotParametersDatabase feels double / overcomplicated. Simply by moving everything to a single struct?
- Referee filter doesn't actually do anything. It's just a wrapper around a proto referee packet. Remove?
- PosVelFilter1D: Maybe rename? It's also used for rotation in RobotOrientationFilter, so name doesn't make sense.
- GeometryFilter: Kinda weird that `SslGeometryData.calib` is stored twice, in `combined_geometry` and `cameras`

```
🆗 Doesn't need to be copied
✅ Copied
❌ Could not copy

├── CMakeLists.txt 🆗
├── include
│   └── roboteam_observer
│       ├── Handler.h
│       └── RobocupReceiver.h
├── observer
│   ├── CMakeLists.txt
│   ├── include
│   │   └── observer
│   │       ├── Observer.h
│   │       ├── data
│   │       │   └── RobotParameters.h ✅
│   │       ├── filters
│   │       │   ├── referee
│   │       │   │   └── RefereeFilter.h ✅
│   │       │   └── vision
│   │       │       ├── Camera.h
│   │       │       ├── CameraMap.h
│   │       │       ├── CameraObjectFilter.h ✅
│   │       │       ├── DetectionFrame.h
│   │       │       ├── GeometryFilter.h ✅
│   │       │       ├── KalmanFilter.h ✅
│   │       │       ├── PosVelFilter1D.h ✅
│   │       │       ├── PosVelFilter2D.h ✅
│   │       │       ├── RobotFeedbackFilter.h
│   │       │       ├── VisionFilter.h
│   │       │       ├── WorldFilter.h
│   │       │       ├── ball
│   │       │       │   ├── BallAssigner.h
│   │       │       │   ├── BallFilter.h
│   │       │       │   ├── BallObservation.h
│   │       │       │   ├── CameraGroundBallFilter.h
│   │       │       │   ├── FilteredBall.h
│   │       │       │   └── GroundBallExtendedKalmanFilter.h
│   │       │       └── robot
│   │       │           ├── CameraRobotFilter.h
│   │       │           ├── FilteredRobot.h
│   │       │           ├── RobotFilter.h
│   │       │           ├── RobotObservation.h
│   │       │           ├── RobotOrientationFilter.h ✅
│   │       │           └── RobotPos.h ✅
│   │       └── parameters
│   │           └── RobotParameterDatabase.h ✅
│   ├── src
│   │   ├── Observer.cpp
│   │   ├── data
│   │   │   └── RobotParameters.cpp ✅
│   │   ├── filters
│   │   │   ├── Scaling.h
│   │   │   ├── referee
│   │   │   │   └── RefereeFilter.cpp ✅
│   │   │   └── vision
│   │   │       ├── Camera.cpp
│   │   │       ├── CameraMap.cpp
│   │   │       ├── CameraObjectFilter.cpp ✅
│   │   │       ├── DetectionFrame.cpp
│   │   │       ├── GeometryFilter.cpp ✅
│   │   │       ├── PosVelFilter1D.cpp ✅
│   │   │       ├── PosVelFilter2D.cpp ✅
│   │   │       ├── RobotFeedbackFilter.cpp
│   │   │       ├── VisionFilter.cpp
│   │   │       ├── WorldFilter.cpp
│   │   │       ├── ball
│   │   │       │   ├── BallAssigner.cpp
│   │   │       │   ├── BallFilter.cpp
│   │   │       │   ├── BallObservation.cpp
│   │   │       │   ├── CameraGroundBallFilter.cpp
│   │   │       │   ├── FilteredBall.cpp
│   │   │       │   └── GroundBallExtendedKalmanFilter.cpp
│   │   │       └── robot
│   │   │           ├── CameraRobotFilter.cpp
│   │   │           ├── FilteredRobot.cpp
│   │   │           ├── RobotFilter.cpp
│   │   │           ├── RobotObservation.cpp
│   │   │           ├── RobotOrientationFilter.cpp ✅
│   │   │           └── RobotPos.cpp ✅
│   │   └── parameters
│   │       └── RobotParameterDatabase.cpp ✅
│   └── test
│       ├── KalmanFilterTest.cpp ❌ Outdated, doesn't align with KalmanFilter.h
│       └── main.cpp 🆗
├── readme.md
└── src
    ├── Handler.cpp
    ├── main.cpp
    └── replayLog.cpp
```