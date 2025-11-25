#!/bin/bash
# Test script for Code Warrior API endpoints

BASE_URL="http://127.0.0.1:3000"

echo "Testing Code Warrior API Endpoints"
echo "===================================="
echo ""

# Health check
echo "1. Health Check (GET /health)"
curl -s "$BASE_URL/health" | jq .
echo ""

# List levels
echo "2. List Levels (GET /api/levels)"
curl -s "$BASE_URL/api/levels" | jq .
echo ""

# Get game state
echo "3. Game State (GET /api/game/state)"
curl -s "$BASE_URL/api/game/state" | jq .
echo ""

# Get render state
echo "4. Render State (GET /api/game/render-state)"
curl -s "$BASE_URL/api/game/render-state" | jq .
echo ""

# Initialize game
echo "5. Initialize Game (POST /api/game/init)"
curl -s -X POST "$BASE_URL/api/game/init" \
  -H "Content-Type: application/json" \
  -d '{}' | jq .
echo ""

# Game action
echo "6. Game Action (POST /api/game/action)"
curl -s -X POST "$BASE_URL/api/game/action" \
  -H "Content-Type: application/json" \
  -d '{"action":"move_north"}' | jq .
echo ""

# Load level
echo "7. Load Level (POST /api/levels/level_1/load)"
curl -s -X POST "$BASE_URL/api/levels/level_1/load" | jq .
echo ""

# Get current level
echo "8. Current Level (GET /api/levels/current)"
curl -s "$BASE_URL/api/levels/current" | jq .
echo ""

# Submit code
echo "9. Submit Code (POST /api/code/submit)"
curl -s -X POST "$BASE_URL/api/code/submit" \
  -H "Content-Type: application/json" \
  -d '{"code":"#include <stdio.h>\nint main() { printf(\"Hello\\n\"); return 0; }","challenge_id":"challenge_1"}' | jq .
echo ""

# Get hint
echo "10. Get Hint (GET /api/code/hint)"
curl -s "$BASE_URL/api/code/hint" | jq .
echo ""

# Complete level
echo "11. Complete Level (POST /api/game/complete-level)"
curl -s -X POST "$BASE_URL/api/game/complete-level" \
  -H "Content-Type: application/json" \
  -d '{"level_id":"level_1","score":850}' | jq .
echo ""

echo "===================================="
echo "All tests complete!"
