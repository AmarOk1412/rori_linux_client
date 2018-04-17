import QtQuick 2.0
import QtQuick.Window 2.0
import QtQuick.Controls 1.1

ApplicationWindow {
    id: root
    width: Screen.width
    height: Screen.height
    visible: true
    visibility: Window.FullScreen
    title: qsTr("RORI For Linux")
    color: "#e04d30"

    property int posY: 4 * Screen.height / 9
    property int endPosY: 7 * height / 18
    property int beginPosY: 4 * Screen.height / 9
    property int arcWidth: Screen.height / 10
    property int animationDuration: 2000
    property int textRoriY: 6 * Screen.height / 10
    property int textUserY: 8 * Screen.height / 10

    onPosYChanged: canvas.requestPaint()

    Behavior on posY {
       id: modifyPosY
       enabled: true
       NumberAnimation {
           duration: root.animationDuration
           easing.type: Easing.InOutCubic
       }
    }

    Behavior on arcWidth {
       id: modifyArcWidth
       enabled: true
       NumberAnimation {
           duration: root.animationDuration
           easing.type: Easing.InOutCubic
       }
    }

    Canvas {
        id: canvas
        width: Screen.width
        height: Screen.height

        onPaint: {
            var ctx = getContext("2d")
            ctx.reset()
            ctx.beginPath()
            var middle = width / 2
            ctx.arc(middle, posY, arcWidth, 0, 2 * Math.PI)
            ctx.strokeStyle = "#ffdad3"
            ctx.lineWidth = height / 100
            ctx.stroke()
            if (posY == endPosY) {
              posY = beginPosY
              arcWidth = height / 10
            } else if (posY == beginPosY) {
              posY = endPosY
              arcWidth = 11 * height / 101
            }

            ctx.beginPath()
            ctx.scale(2, 0.1)
            var yShadow = 70 * height / 9
            var radiusShadow = arcWidth / 1.5
            ctx.arc(middle / 2, yShadow, radiusShadow, 0, 2*Math.PI)
            var gradient = ctx.createRadialGradient(middle / 2, yShadow,
                                                    radiusShadow / 10,
                                                    middle / 2, yShadow,
                                                    radiusShadow);
            gradient.addColorStop(0, 'rgba(0, 0, 0, 0.1)');
            gradient.addColorStop(1, 'transparent');
            ctx.fillStyle = gradient;
            ctx.fill()
        }
    }

    Text {
        id: textRori
        text: "Hei, it's been a long time!"
        font.family: "Deja Vu"
        y: textRoriY
        x: Screen.width / 2 - width / 2
        font.pointSize: 35
        color: "#ffdad3"
        opacity: 0

        SequentialAnimation on opacity {
            loops: Animation.Infinite
            NumberAnimation { from: 0; to: 1; duration: 500 }
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: 1; to: 0; duration: 500 }
            PauseAnimation { duration: 2000 }
        }

        SequentialAnimation on y {
            loops: Animation.Infinite
            NumberAnimation { from: textRoriY - 100; to: textRoriY; duration: 500 }
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: textRoriY; to: textRoriY - 100; duration: 500 }
            PauseAnimation { duration: 2000 }
        }
    }

    Text {
        id: textUser
        font.family: "Deja Vu"
        text: "I want some cake! I want some cake! I want some cake! I want some cake! I want some cake! I want some cake!\nI want some cake! I want some cake! I want some cake! I want some cake! I want some cake! I want some cake!"
        y: textUserY
        x: Screen.width / 2 - width / 2
        font.pointSize: 35
        color: "#ffdad3"
        opacity: 0

        SequentialAnimation on opacity {
            loops: Animation.Infinite
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: 0; to: 1; duration: 500 }
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: 1; to: 0; duration: 500 }
        }

        SequentialAnimation on y {
            loops: Animation.Infinite
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: textUserY + 100; to: textUserY; duration: 500 }
            PauseAnimation { duration: 2000 }
            NumberAnimation { from: textUserY; to: textUserY + 100; duration: 500 }
        }
    }

    SequentialAnimation on color {
        loops: Animation.Infinite
        ColorAnimation { from: "#e04d30"; to: "#e07730"; duration: 5000 }
        ColorAnimation { from: "#e07730"; to: "#e03d30"; duration: 5000 }
        ColorAnimation { from: "#e03d30"; to: "#e04d30"; duration: 5000 }
    }
}
