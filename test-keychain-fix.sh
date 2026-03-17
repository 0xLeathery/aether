#!/bin/bash
# Test procedure for keychain password prompt fix

echo "=== Keychain Fix Test Procedure ==="
echo ""
echo "Step 1: Delete existing keychain items"
security delete-generic-password -s "com.aether.identity" -a "secret_key" 2>/dev/null
security delete-generic-password -s "com.aether.identity" -a "display_name" 2>/dev/null
echo "✓ Keychain items deleted (if they existed)"
echo ""

echo "Step 2: Launch the app with 'npm run tauri dev'"
echo "  - You may see a password prompt (normal for first-time creation)"
echo "  - Complete the setup flow"
echo "  - The ACL will be set automatically"
echo ""
echo "Step 3: Close the app"
echo ""
echo "Step 4: Launch the app again with 'npm run tauri dev'"
echo "  - EXPECTED: NO password prompt"
echo "  - FAILURE: Password prompt appears"
echo ""
echo "Step 5: Verify ACL was set (run this command):"
echo "  security find-generic-password -s 'com.aether.identity' -a 'secret_key'"
echo "  Look for 'applications:' section with your app executable"
echo ""
