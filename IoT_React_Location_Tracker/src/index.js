import React from "react"; // Importa a biblioteca React, necessária para criar componentes e elementos React.
import ReactDOM from "react-dom"; // Importa o ReactDOM para renderizar componentes React no DOM.
import "./App.css"; // Importa o arquivo de estilos CSS para aplicar à aplicação.
import App from "./App"; // Importa o componente principal da aplicação, que será renderizado.

ReactDOM.render(
  <React.StrictMode> {/* Habilita o modo estrito do React, que ajuda a identificar problemas potenciais na aplicação durante o desenvolvimento. */}
    <App /> {/* Renderiza o componente principal da aplicação. */}
  </React.StrictMode>,
  document.getElementById("root") // Seleciona o elemento HTML com o ID "root" como ponto de montagem da aplicação React.
);
