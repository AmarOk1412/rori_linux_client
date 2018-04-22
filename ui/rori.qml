import QtQuick 2.7
import QtQuick.Window 2.0
import QtQuick.Controls 1.3
import QtQuick.Controls.Styles 1.4

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
    property bool logged: false
    property int loggedStep: 0
    property int opacityR: 255

    onPosYChanged: canvas.requestPaint()
    onOpacityRChanged: canvas.requestPaint()

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

    Behavior on opacityR {
       id: modifyOpacity
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
            if (logged) {
              opacityR = 255
              if (posY == endPosY) {
                posY = beginPosY
                arcWidth = height / 10
              } else if (posY == beginPosY) {
                posY = endPosY
                arcWidth = 11 * height / 101
              }
            } else {
              if (opacityR == 0) {
                opacityR = 255
              } else if (opacityR == 255) {
                opacityR = 0
              }
              if (posY != 2 * Screen.height / 9) {
                posY = 2 * Screen.height / 9;
              }
            }
            var ctx = getContext("2d")
            ctx.reset()
            ctx.beginPath()
            var middle = width / 2
            ctx.arc(middle, posY, arcWidth, 0, 2 * Math.PI)
            ctx.strokeStyle = 'rgba(255, 218, 211, ' + opacityR/255. +')'
            ctx.lineWidth = height / 100
            ctx.stroke()

            ctx.beginPath()
            ctx.scale(2, 0.1)
            var yShadow = 70 * height / 9
            var radiusShadow = arcWidth / 1.5
            ctx.arc(middle / 2, yShadow, radiusShadow, 0, 2*Math.PI)
            var gradient = ctx.createRadialGradient(middle / 2, yShadow,
                                                    radiusShadow / 10,
                                                    middle / 2, yShadow,
                                                    radiusShadow);
            gradient.addColorStop(0, 'rgba(0, 0, 0, ' + opacityR/2550. +')');
            gradient.addColorStop(1, 'transparent');
            ctx.fillStyle = gradient;
            ctx.fill()
        }
    }

    Text {
        id: textRori
        text: ""
        font.family: "Deja Vu"
        y: textRoriY - 100
        opacity: 0
        width: 9 * Screen.width / 10
        horizontalAlignment: TextEdit.AlignHCenter
        wrapMode: Text.Wrap
        x: Screen.width / 2 - width / 2
        font.pointSize: 35
        color: "#ffdad3"
    }


    NumberAnimation {
        id: unshowRORIText
        properties: "opacity"
        target: textRori
        to: 0.0
        easing.type: Easing.InOutQuad
        duration: 500
    }

    NumberAnimation {
        id: downRORIText
        target: textRori
        properties: "y"
        to: textRoriY
        easing.type: Easing.InOutQuad
        duration: 500
    }

    NumberAnimation {
       id: showRORIText
       target: textRori
       properties: "opacity"
       to: 1.0
       easing.type: Easing.InOutQuad
       duration: 500
    }

    NumberAnimation {
        id: upRORIText
        target: textRori
        properties: "y"
        to: textRoriY - 100
        easing.type: Easing.InOutQuad
        duration: 500
    }

    TextField {
      id: textUser
      font.family: "Deja Vu"
      y: textUserY
      width: Screen.width
      height: Screen.height - textUserY
      horizontalAlignment: TextEdit.AlignHCenter
      font.pointSize: 35
      inputMethodHints: Qt.ImhMultiLine
      focus: true

      style: TextFieldStyle {
        textColor: "#ffdad3"

        background: Rectangle {
            color: "transparent"
            radius: 0
            border.width: 0
        }
      }

      Keys.onPressed: {
        if (!logged) return
        unshowRORIText.start()
        upRORIText.start()
      }

      Keys.onReturnPressed: {
        if (!logged) {
          unshowRORIText.start()
          upRORIText.start()
        }
        sharedprop.set_user_text(text)
        text = ""
      }
    }

    Timer {
        interval: 250; running: true; repeat: true
        onTriggered: {
          if (!logged) {
            logged = sharedprop.get_logged()
            if (logged) {
              posY = beginPosY
            }
          }

          var new_rori_text = sharedprop.get_rori_text()
          if (new_rori_text != textRori.text) {
            textRori.text = new_rori_text
            showRORIText.start()
            downRORIText.start()
          }
        }
    }


    SequentialAnimation on color {
        loops: Animation.Infinite
        ColorAnimation { from: "#e04d30"; to: "#e07730"; duration: 5000 }
        ColorAnimation { from: "#e07730"; to: "#e03d30"; duration: 5000 }
        ColorAnimation { from: "#e03d30"; to: "#e04d30"; duration: 5000 }
    }
}
