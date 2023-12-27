import { Sudoku } from "sudoku-rust-wasm";

let sudokuObject = new Sudoku();

function onInput(event, row, col) {
  if (event.target.value > 1) {
    event.target.value = event.target.value.slice(0, 1);
  }
  sudokuObject.input(row, col, event.target.value || 0);
  document.getElementById(`cell-${row}-${col}`).classList.remove("incorrect");
  if (sudokuObject.has_won()) {
    alert("You won!");
  }
}

function initializeSudoku() {
  const newSudoku = Array.from({ length: 9 }, (_, i) =>
    sudokuObject.get_puzzle().slice(i * 9, (i + 1) * 9)
  );
  const isPortrait = window.matchMedia("(orientation: portrait)").matches;

  const table = document.getElementById("sudokuTable");
  for (let i = 0; i < 9; i++) {
    const row = table.insertRow();

    for (let j = 0; j < 9; j++) {
      const cell = row.insertCell();
      cell.classList.add("cell");
      cell.id = `cell-${i}-${j}`;
      if (newSudoku[i][j] === "0") {
        const input = document.createElement("input");
        input.id = `input-${i}-${j}`;
        input.setAttribute("type", "number");
        input.setAttribute("min", "1");
        input.setAttribute("max", "9");
        input.addEventListener("input", (event) => onInput(event, i, j));
        if (isPortrait) {
          input.style.marginRight = "0";
        }
        cell.appendChild(input);
      } else {
        cell.textContent = newSudoku[i][j];
        cell.classList.add("read-only");
      }
      if ((j + 1) % 3 === 0 && j < 8) {
        cell.classList.add("border-right");
      }
    }

    if ((i + 1) % 3 === 0 && i < 8) {
      row.classList.add("border-bottom");
    }
  }
}

document.getElementsByClassName("button-new")[0].onclick = function () {
  document.getElementById("sudokuTable").innerHTML = "";
  sudokuObject = new Sudoku();
  initializeSudoku();
};

document.getElementsByClassName("button-next-step")[0].onclick = function () {
  const nestStep = sudokuObject.next_step();
  if (!nestStep) {
    if (sudokuObject.has_won()) {
      alert("Cangratulations! You won!");
    } else {
      alert("Next step not found! Check if all your numbers are correct.");
    }
  } else {
    const inputField = document.getElementById(
      `input-${nestStep.row}-${nestStep.col}`
    );
    inputField.value = nestStep.value;
  }
};

document.getElementsByClassName("button-check")[0].onclick = function () {
  sudokuObject.incorrect_fields().forEach((index) => {
    const row = Math.floor(index / 9);
    const col = index % 9;
    const cell = document.getElementById(`cell-${row}-${col}`);
    cell.classList.add("incorrect");
  });
};

window.onload = initializeSudoku();
