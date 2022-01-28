# I2NP

The Invisible Internet Network Protocol (I2NP) is a networking layer that resides above the I2P Transport Protocol layer. The I2NP layer manages the routing of inter-router communications, as well as self determining which transport protocol to use if there are multiple transport protocols to choose from.

> "I2NP (I2P Network Protocol) messages can be used for one-hop, router-to-router, point-to-point messages. By encrypting and wrapping messages in other messages, they can be sent in a secure way through multiple hops to the ultimate destination." [1]

> "All routers must publish their I2NP protocol version in the "router.version" field in the RouterInfo properties. This version field is the API version, indiciating the level of support for various I2NP protocol features, and is not necessarily the actual router version." [2]

## Supplemental Information

1. [geti2p.net I2NP Protocol Documentation](https://geti2p.net/en/docs/protocol/i2np)
2. [geti2p.net I2NP Technical Specification](https://geti2p.net/spec/i2np)
