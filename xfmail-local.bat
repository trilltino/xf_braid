@echo off
title XFMail Backend Server
color 0A
echo ========================================
echo   XFMail Backend Server - Local Dev
echo ========================================
echo.
echo Starting backend server on http://localhost:3000
echo Press Ctrl+C to stop the server
echo.
echo Database: Using SQLite (data.db in xfmail/backend directory)
echo.
cd xfmail\backend
cargo run --features ssr
pause
