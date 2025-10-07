zellij:
  #!/usr/bin/env bash
  # Check if a Zellij session named "multi" exists
  if zellij ls | grep -q "trading_bot"; then
      zellij kill-session trading_bot
      zellij delete-session trading_bot
  fi

  zellij -s trading_bot --layout layout.kdl
