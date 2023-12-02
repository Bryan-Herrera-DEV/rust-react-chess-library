import { useEffect, useState } from 'react'
import './App.css'
import init, { ChessBoard } from 'chessboard_wasm';

type Cell = string | null;
type Board = Cell[][];

function App() {
  const [board, setBoard] = useState<Board>([]);

  const convertTo2D = (linearArray: number[], rowSize: number): Board => {
    return linearArray.reduce((rows, key, index) => {
      if (index % rowSize === 0) {
        rows.push([key]);
      } else {
        rows[rows.length - 1].push(key);
      }
      return rows;
    }, [] as Board);
  };


  useEffect(() => {
    init().then(() => {
      const chessBoard = ChessBoard.new();
      const linearBoard = chessBoard.get_board(); // Obtiene el tablero como un array lineal
      setBoard(convertTo2D(linearBoard, 8)); // Convierte a 2D y actualiza el estado
    });
  }, []);
  return (
    <div className="chessboard">
      {board.map((row, rowIndex) => (
        <div key={rowIndex} className="chess-row">
          {row.map((cell, cellIndex) => (
            <div key={cellIndex} className={`chess-cell ${cell === 0 ? 'empty' : 'occupied'}`}>
              {cell !== 0 && `Piece ${cell}`} {/* Representa la pieza aquí */}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}

export default App
