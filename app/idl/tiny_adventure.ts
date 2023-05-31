export type TinyAdventure = {
  "version": "0.1.0",
  "name": "tiny_adventure",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "newGameDataAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveLeft",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveRight",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveUp",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveDown",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "gameDataAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "playerPositionX",
            "type": "u8"
          },
          {
            "name": "playerPositionY",
            "type": "u8"
          }
        ]
      }
    }
  ]
};

export const IDL: TinyAdventure = {
  "version": "0.1.0",
  "name": "tiny_adventure",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "newGameDataAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveLeft",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveRight",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveUp",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "moveDown",
      "accounts": [
        {
          "name": "gameDataAccount",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "gameDataAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "playerPositionX",
            "type": "u8"
          },
          {
            "name": "playerPositionY",
            "type": "u8"
          }
        ]
      }
    }
  ]
};