//
//  main.swift
//  racer_cli
//
//  Created by Rakhasa on 25/02/23.
//

import Foundation

struct PlayerModel {
    var icon : String
    var position : Int
}

print("Racer CLI")
print("Input row and col : ", terminator: "")
var input = readLine() ?? ""

var getRowCol : [String] = input.components(separatedBy: " ")

let rows : Int = Int(getRowCol[0]) ?? 1
let cols : Int = Int(getRowCol[1]) ?? 1

var players : [PlayerModel] = []

for rowIndex in 0...rows - 1 {
    players.append(PlayerModel(icon: String(UnicodeScalar(UInt8(65 + rowIndex))), position: 0))
}

var isFinish : Bool = false

func showUi (){
    if (players.filter{player in player.position == cols - 1}.count) > 0 {
        isFinish = true
    }
    
    for row in 0...rows - 1 {

        var road : String =  ""
        let position : Int = players[row].position
        let icon : String = players[row].icon

        for col in 0...cols - 1 {

            if col == position {
                road.append("|\(icon)")
            }
            else {
                road.append("| ")
            }

        }
        
        print(road)
        
        if !isFinish {
            let getRandomPosition : Int = position + Int.random(in: 1...3)
            players[row].position = getRandomPosition > cols - 1 ? cols - 1 : getRandomPosition
        }
    }
}

while (!isFinish) {
    let cls = Process()
    let out = Pipe()
    cls.launchPath = "/usr/bin/clear"
    cls.standardOutput = out
    cls.launch()
    cls.waitUntilExit()
    print (String(data: out.fileHandleForReading.readDataToEndOfFile(), encoding: String.Encoding.utf8) ?? "")
    
    showUi()
    
    sleep(1)
}

let iconWinner : String = players.filter{ player in player.position == cols - 1}.first?.icon ?? ""
print("Player \(iconWinner) is the winner")
