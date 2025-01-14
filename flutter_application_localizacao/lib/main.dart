import 'package:flutter/material.dart';
import 'package:geolocator/geolocator.dart';
import 'package:mqtt_client/mqtt_client.dart';
import 'package:mqtt_client/mqtt_server_client.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: LocationSenderScreen(),
    );
  }
}

class LocationSenderScreen extends StatefulWidget {
  @override
  _LocationSenderScreenState createState() => _LocationSenderScreenState();
}

class _LocationSenderScreenState extends State<LocationSenderScreen> {
  late MQTTService _mqttService;
  String statusMessage = "Pressione o botão para enviar a localização.";

  @override
  void initState() {
    super.initState();
    _mqttService = MQTTService(
      onStatusUpdate: (message) {
        setState(() {
          statusMessage = message;
        });
      },
    );
    _mqttService.connect();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Envio de Localização via MQTT"),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(
                statusMessage,
                textAlign: TextAlign.center,
              ),
              SizedBox(height: 20),
              ElevatedButton(
                onPressed: () async {
                  Position? position =
                      await LocationService.getCurrentLocation();
                  if (position != null) {
                    String location =
                        "Lat: ${position.latitude}, Lng: ${position.longitude}";
                    _mqttService.publishLocation(location);
                  } else {
                    setState(() {
                      statusMessage = "Erro ao obter localização.";
                    });
                  }
                },
                child: Text("Enviar Localização"),
              ),
            ],
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    _mqttService.disconnect();
    super.dispose();
  }
}

class MQTTService {
  final Function(String) onStatusUpdate;
  late MqttServerClient client;
  String broker =
      "test.mosquitto.org"; // Substitua pelo endereço do seu broker MQTT
  int port = 1883;
  String topic = "device/location";

  MQTTService({required this.onStatusUpdate});

  void connect() async {
    client = MqttServerClient(broker, 'flutter_client');
    client.port = port;
    client.keepAlivePeriod = 20;
    client.onDisconnected = _onDisconnected;
    client.logging(on: true);

    try {
      await client.connect();
      onStatusUpdate("Conectado ao broker MQTT.");
    } catch (e) {
      onStatusUpdate("Falha ao conectar ao broker: $e");
    }
  }

  void _onDisconnected() {
    onStatusUpdate("Desconectado do broker MQTT.");
  }

  void publishLocation(String location) {
    if (client.connectionStatus?.state == MqttConnectionState.connected) {
      final builder = MqttClientPayloadBuilder();
      builder.addString(location);

      client.publishMessage(topic, MqttQos.exactlyOnce, builder.payload!);
      onStatusUpdate("Localização enviada: $location");
    } else {
      onStatusUpdate("Não está conectado ao broker MQTT.");
    }
  }

  void disconnect() {
    client.disconnect();
  }
}

class LocationService {
  static Future<Position?> getCurrentLocation() async {
    bool serviceEnabled;
    LocationPermission permission;

    serviceEnabled = await Geolocator.isLocationServiceEnabled();
    if (!serviceEnabled) {
      return null;
    }

    permission = await Geolocator.checkPermission();
    if (permission == LocationPermission.denied) {
      permission = await Geolocator.requestPermission();
      if (permission == LocationPermission.denied) {
        return null;
      }
    }

    if (permission == LocationPermission.deniedForever) {
      return null;
    }

    return await Geolocator.getCurrentPosition();
  }
}
