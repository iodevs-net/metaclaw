#!/usr/bin/env python3
"""
Cliente WebSocket completo para metaclaw.
Hace pairing automático y permite enviar mensajes al agente.

Uso:
    python scripts/ws_metaclaw.py "tu mensaje aquí"
"""

import asyncio
import json
import sys
import time
import re
import subprocess
import urllib.request
import urllib.error
import websocket

class MetaclawWSClient:
    def __init__(self, host="127.0.0.1", port=42777):
        self.host = host
        self.port = port
        self.base_url = f"http://{host}:{port}"
        self.ws_url = f"ws://{host}:{port}/ws/chat"
        self.token = None
        self.session_id = None
        self.ws = None
        
    def get_latest_pairing_code(self):
        """Obtiene el código de pairing más reciente de los logs del contenedor."""
        result = subprocess.run(
            ["docker", "logs", "metaclaw-dev"],
            capture_output=True, text=True
        )
        # Buscar códigos de 6 dígitos en formato visual: │  123456  │
        codes = re.findall(r'│\s*(\d{6})\s*│', result.stdout)
        if codes:
            return codes[-1]  # El último código
        return None
    
    def pair(self):
        """Hace pairing HTTP para obtener token."""
        code = self.get_latest_pairing_code()
        if not code:
            raise Exception("No se encontró código de pairing en los logs")
        
        print(f"🔐 Código de pairing: {code}")
        
        req = urllib.request.Request(
            f"{self.base_url}/pair",
            method="POST",
            headers={"X-Pairing-Code": code}
        )
        
        try:
            with urllib.request.urlopen(req, timeout=10) as response:
                data = json.loads(response.read().decode())
                self.token = data.get("token")
                if self.token:
                    print(f"✅ Pairing exitoso")
                    return self.token
                else:
                    raise Exception(f"No se recibió token: {data}")
        except urllib.error.HTTPError as e:
            error_body = e.read().decode()
            raise Exception(f"Pairing falló ({e.code}): {error_body}")
    
    def connect_ws(self):
        """Conecta al WebSocket con el token."""
        # Generar session ID único
        self.session_id = f"ws_{int(time.time())}_{id(self)}"
        
        # Conectar
        self.ws = websocket.WebSocketApp(
            f"{self.ws_url}?session_id={self.session_id}&token={self.token}",
            on_message=self.on_message,
            on_error=self.on_error,
            on_close=self.on_close,
            on_open=self.on_open
        )
        
        print(f"🔌 Conectando a {self.ws_url}...")
        self.ws.run_forever(ping_interval=30, ping_timeout=10)
    
    def on_open(self, ws):
        print("✅ WebSocket conectado")
        # Enviar mensaje
        message = self.message_to_send
        print(f"\n📤 Enviando: {message[:100]}...")
        ws.send(json.dumps({
            "type": "message",
            "content": message
        }))
    
    def on_message(self, ws, message):
        data = json.loads(message)
        msg_type = data.get("type")
        
        if msg_type == "session_start":
            print(f"📍 Sesión: {data.get('session_id')}, resuming: {data.get('resumed')}")
        elif msg_type == "connected":
            print(f"✅ {data.get('message')}")
        elif msg_type == "chunk":
            text = data.get("content", "")
            if text:
                print(text, end="", flush=True)
        elif msg_type == "thinking":
            print(f"\n💭 {data.get('content', '')}", flush=True)
        elif msg_type == "tool_call":
            name = data.get("name", "")
            args = data.get("args", {})
            print(f"\n\n🔧 === HERRAMIENTA: {name} ===")
            if args:
                print(f"   Args: {json.dumps(args, indent=4, ensure_ascii=False)}")
        elif msg_type == "tool_result":
            output = data.get("output", "")
            if len(output) > 500:
                output = output[:500] + "..."
            print(f"\n📥 Resultado: {output}")
        elif msg_type == "done":
            full = data.get("full_response", "")
            print(f"\n\n═══════════════════════════════════════")
            print(f"✅ RESPUESTA FINAL:")
            print(f"═══════════════════════════════════════")
            print(full[:2000] if len(full) > 2000 else full)
            print(f"═══════════════════════════════════════")
            self.ws.close()
        elif msg_type == "error":
            print(f"\n❌ ERROR: {data.get('message', '')}")
            if data.get("code"):
                print(f"   Código: {data.get('code')}")
        elif msg_type == "chunk_reset":
            pass  # Ignore reset messages
        else:
            print(f"\n[{msg_type}]: {data}")
    
    def on_error(self, ws, error):
        print(f"\n❌ Error WebSocket: {error}")
    
    def on_close(self, ws, code, reason):
        print(f"\n🔌 Conexión cerrada ({code}): {reason}")
    
    def send_message(self, message):
        """Envía un mensaje al agente."""
        self.message_to_send = message
        
        # Primero hacer pairing
        self.pair()
        
        # Luego conectar WebSocket
        self.connect_ws()

def main():
    if len(sys.argv) < 2:
        print("╔════════════════════════════════════════════════════╗")
        print("║     Cliente WebSocket para Metaclaw              ║")
        print("╠════════════════════════════════════════════════════╣")
        print("║  Uso: python scripts/ws_metaclaw.py \"mensaje\"   ║")
        print("╚════════════════════════════════════════════════════╝")
        sys.exit(1)
    
    message = sys.argv[1]
    
    client = MetaclawWSClient()
    try:
        client.send_message(message)
    except KeyboardInterrupt:
        print("\n\n⚠️  Interrupted by user")
        sys.exit(0)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
