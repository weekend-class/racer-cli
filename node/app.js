const createReadlineInterface = require("./lib/readline-interface");

console.log("Racer CLI");
console.log("");
createReadlineInterface(main);

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function initPlayers(totalPlayer) {
  return Array(totalPlayer)
    .fill(0)
    .map((_, i) => ({
      name: String.fromCharCode(97 + i),
      // position: i === 0 ? 20 : 0,
      position: 0,
    }));
}

function clearScreen() {
  console.clear();
}

function getRandomMovement(max) {
  return Math.floor(Math.random() * max);
}

function createTrackBoard(players = [], totalTrack = 0) {
  clearScreen();
  let race = "";

  players.forEach((player) => {
    for (let track = 0; track <= totalTrack; track++) {
      if (player.position === track) {
        race += `|${player.name}`;
      } else {
        race += "| ";
      }
    }
    race += "\n";
  });

  console.log(race);
}

function getTheWinner(players, trackLength) {
  return players.find((player) => player.position === trackLength);
}

function printWinner(winner) {
  console.log(`Player ${winner.name} is the winner!!!`);
}

async function createRace(totalPlayer, trackLength) {
  let players = initPlayers(totalPlayer);

  while (!getTheWinner(players, trackLength)) {
    await sleep(500);
    createTrackBoard(players, trackLength);
    players = players.map((v) => ({
      name: v.name,
      position: getRandomMovement(2) + v.position,
    }));
  }
  let winner = getTheWinner(players, trackLength);

  printWinner(winner);
  process.exit();
}

async function main(input) {
  clearScreen();
  const inputGame = input.split(" ").map(Number);

  if (inputGame.length !== 2) {
    console.log("Minimum argument should be 2");
    return;
  }

  console.log("Gearing up, the race is about to start...");
  await sleep(500);
  console.log("Start...");
  await sleep(500);
  clearScreen();

  const totalPlayer = inputGame[0];
  const trackLength = inputGame[1];

  createRace(totalPlayer, trackLength);
}
