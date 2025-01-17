import React, { useEffect, useState } from "react"; // Importa React, os hooks useEffect e useState para gerenciar estado e efeitos colaterais.
import mqtt from "mqtt"; // Importa a biblioteca MQTT para comunicação com o broker.
import axios from "axios"; // Importa Axios para realizar requisições HTTP.
import "./App.css"; // Importa o arquivo CSS para estilização.

const App = () => {
  // Define os estados do componente
  const [location, setLocation] = useState({ lat: null, lon: null }); // Armazena a localização do usuário.
  const [brokerStatus, setBrokerStatus] = useState("Disconnected"); // Armazena o status da conexão com o broker MQTT.
  const [orionData, setOrionData] = useState(null); // Armazena dados obtidos do Orion Context Broker.

  // Hook para conexão com o broker MQTT e captura da localização
  useEffect(() => {
    const client = mqtt.connect("ws://localhost:9001"); // Conecta ao broker MQTT via WebSocket.

    client.on("connect", () => {
      setBrokerStatus("Connected"); // Atualiza o status quando conectado ao broker.
    });

    client.on("error", (err) => {
      console.error("MQTT Connection Error:", err); // Loga erros de conexão.
      setBrokerStatus("Disconnected"); // Atualiza o status em caso de erro.
    });

    // Verifica se a API de geolocalização está disponível no navegador
    if (navigator.geolocation) {
      navigator.geolocation.watchPosition(
        (position) => {
          const { latitude, longitude } = position.coords; // Obtém latitude e longitude do usuário.
          setLocation({ lat: latitude, lon: longitude }); // Atualiza o estado da localização.

          // Publica os dados de localização no tópico MQTT "iot/location"
          const payload = JSON.stringify({
            id: "user1", // ID do usuário ou dispositivo.
            type: "Location",
            latitude,
            longitude,
          });

          client.publish("iot/location", payload); // Envia os dados para o broker.
        },
        (error) => console.error("Geolocation Error:", error), // Loga erros de geolocalização.
        { enableHighAccuracy: true } // Solicita maior precisão na localização.
      );
    } else {
      alert("Geolocation is not supported by this browser."); // Alerta caso a API de geolocalização não seja suportada.
    }

    return () => client.end(); // Encerra a conexão com o broker ao desmontar o componente.
  }, []);

  // Hook para buscar dados do Orion Context Broker
  useEffect(() => {
    const fetchOrionData = async () => {
      try {
        // Faz uma requisição GET para obter dados de uma entidade no Orion
        const response = await axios.get(
          "http://localhost:1026/v2/entities/user1"
        );
        setOrionData(response.data); // Atualiza o estado com os dados recebidos.
      } catch (error) {
        console.error("Error fetching data from Orion:", error); // Loga erros na requisição.
      }
    };

    const interval = setInterval(fetchOrionData, 5000); // Atualiza os dados a cada 5 segundos.

    return () => clearInterval(interval); // Limpa o intervalo ao desmontar o componente.
  }, []);

  return (
    <div className="App">
      <h1>IoT Location Tracker</h1>
      <p><strong>MQTT Broker Status:</strong> {brokerStatus}</p> {/* Exibe o status do broker. */}

      <h2>Your Location</h2>
      {location.lat && location.lon ? ( // Verifica se a localização foi obtida.
        <p>
          Latitude: {location.lat} <br /> Longitude: {location.lon}
        </p>
      ) : (
        <p>Fetching location...</p> // Exibe mensagem enquanto a localização está sendo obtida.
      )}

      <h2>Data from Orion Context Broker</h2>
      {orionData ? ( // Verifica se os dados do Orion foram recebidos.
        <pre>{JSON.stringify(orionData, null, 2)}</pre> // Exibe os dados em formato JSON.
      ) : (
        <p>Fetching data from Orion...</p> // Exibe mensagem enquanto os dados estão sendo buscados.
      )}
    </div>
  );
};

export default App; // Exporta o componente para ser utilizado em outros módulos.
