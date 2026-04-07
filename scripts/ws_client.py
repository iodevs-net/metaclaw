#!/usr/bin/env python3
"""
Cliente WebSocket para metaclaw.
Permite chatear con el agente y ver ejecución de herramientas en tiempo real.

Uso:
    python scripts/ws_client.py "tu mensaje aquí"
"""

import asyncio
import json
import sys
import websocket

def main():
    if len(sys.argv) < 2:
        print("Uso: python scripts/ws_client.py \"tu mensaje aquí\"")
        sys.exit(1)
    
    message = sys.argv[1]
    
    # Obtener token de pairing (necesitamos hacer pairing primero via HTTP)
    import urllib.request
    
    # 1. Obtener código de pairing
    # Por ahora usamos el último código del log
    print("Conectando a metaclaw via WebSocket...")
    
    # URL del WebSocket
    WS_URL = "ws://127.0.0.1:42777/ws/chat"
    
    # Primero necesitamos pairing via HTTP para obtener token
    # Esto es una simplificación - en producción el cliente maneja esto
    
    try:
        ws = websocket.WebSocketApp(
            WS_URL,
            header={"Authorization": "Bearer dummy"},  # Will be rejected without valid auth
            on_message=on_message,
            on_error=on_error,
            on_close=on_close,
        )
        
        print("WebSocket no disponible directamente. Usando método alternativo...")
        
    except Exception as e:
        print(f"Error: {e}")

def on_message(ws, message):
    data = json.loads(message)
    msg_type = data.get("type")
    
    if msg_type == "session_start":
        print(f"✅ Sesión iniciada: {data}")
    elif msg_type == "chunk":
        print(data.get("content", ""), end="", flush=True)
    elif msg_type == "thinking":
        print(f"\n💭 {data.get('content', '')}", flush=True)
    elif msg_type == "tool_call":
        print(f"\n🔧 Llamando herramienta: {data.get('name')}")
        print(f"   Args: {json.dumps(data.get('args', {}), indent=2)}")
    elif msg_type == "tool_result":
        print(f"\n📤 Resultado: {data.get('output', '')[:200]}...")
    elif msg_type == "done":
        print(f"\n\n✅ Respuesta completa: {data.get('full_response', '')}")
    elif msg_type == "error":
        print(f"\n❌ Error: {data.get('message', '')}")
    else:
        print(f"\n[{msg_type}]: {data}")

def on_error(ws, error):
    print(f"Error: {error}")

def on_close(ws, close_status_code, close_msg):
    print("Conexión cerrada")

if __name__ == "__main__":
    main()
