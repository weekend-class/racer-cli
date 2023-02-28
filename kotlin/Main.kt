package todolist
import kotlin.random.Random

data class Players(var name: String, var position: Int)

var listPosition = arrayListOf<Int>()
val firstStep = 0

fun main(args: Array<String>) {
    println("Racer CLI")
    println("")
    println("Please input players and track length :")

    var input = readLine()

    while (input != null) {
        letTheRaceBegin(input)
        input  = readLine()
    }

}

//fun Sleep(){
//    Thread.sleep(1000)
//}

fun ClearScreen(){
    print("\u001b[H\u001b[2J")
}

fun createTrack(jmlPemain: List<Players>, distance: Int){
    ClearScreen()
    println(jmlPemain)
//    jmlPemain.forEach{
//        for(i in 0..jmlPemain.size){
//            if(jmlPemain[i].position == distance){
//                print(jmlPemain[i].name)
//            }else{
//                print("| ")
//            }
//        }
//        println("")
//    }
    for(i in 0..jmlPemain.size-1){
        if(jmlPemain[i].position == 0){
            print(jmlPemain[i].name)
        }
        for(j in 1..distance){
            print("| ")
            if(jmlPemain[i].position == j){
                print(jmlPemain[i].name)
            }
        }
        println("")
    }
}
//    for(i in 0..jmlPemain.size){
//        if(jmlPemain[i].position == 0){
//            print(jmlPemain[i].name)
//        }
//        for(j in 1..distance){
//            print("| ")
//        }
//        println("")
//    }
//}

fun getTheWinner(players: List<Players>, totalLength: Int): Boolean {
//    return players.find{it.position == totalLength}
    var al = players.find { p ->p.position >= totalLength}
    return al != null

}

//fun printTheWinner(winner){
//   println("The winner is player ${winner.name}")
//}

fun createPlayer(input : Int?): List<Players> {
    val players = mutableListOf<Players>()
    var ascii = 97
    for(i in 1..input!!){
        players.add(Players("${(input+ascii).toChar()}",0))
        ascii++
    }

    return players

}

fun letTheRaceBegin(input : String) {
//    val pemain = mutableListOf<Players>()

    val splitStrings = input.split(" ").toMutableList()
    var jumlahPemain = createPlayer(splitStrings[0].toInt())
    val distance = splitStrings[1].toInt()

//    println(getTheWinner(jumlahPemain, distance))

    while (getTheWinner(jumlahPemain, distance) == false) {
        createTrack(jumlahPemain, distance)
        jumlahPemain.map {
            it.position = it.position+ Random.nextInt(1, 3)
            if(it.position >= distance){
                println("The winner is player ${it.name}")
            }
        }

//        var pemenang = getTheWinner(jumlahPemain,distance)
//        printTheWinner(pemenang)

    }
}
//    createTrack(jumlahPemain,distance)
//    jumlahPemain.map { it.position + Random.nextInt(1,3)}
//}
