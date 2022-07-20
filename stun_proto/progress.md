# Implementation progress of RFC standards

This documents tracks the implementation progress of the STUN features listed in all RFC documents.

Legend:

&check; - implemented

&cross; - not implemented yet [Want to help?]()

## [RFC 3489](https://datatracker.ietf.org/doc/html/rfc3489)

### Message types
| Type                           | Reader  | Writer  |
|:-------------------------------|:-------:|:-------:|
| `Binding Request`              | &check; | &check; |
| `Binding Response`             | &check; | &check; |
| `Binding Error Response`       | &check; | &check; |
| `Shared Secret Request`        | &check; | &check; |
| `Shared Secret Response`       | &check; | &check; |
| `Shared Secret Error Response` | &check; | &check; |

### Header features

| Feature        | Reader  | Writer  |
|:---------------|:-------:|:-------:|
| Message length | &check; | &check; |
| Transaction ID | &check; | &check; |

### Attributes

| Attribute            | Reader  | Writer  |
|:---------------------|:-------:|:-------:|
| `MAPPED-ADDRESS`     | &check; | &check; |
| `RESPONSE-ADDRESS`   | &check; | &check; |
| `CHANGE-REQUEST`     | &check; | &check; |
| `SOURCE-ADDRESS`     | &check; | &check; |
| `CHANGED-ADDRESS`    | &check; | &check; |
| `USERNAME`           | &check; | &cross; |
| `PASSWORD`           | &check; | &cross; |
| `MESSAGE-INTEGRITY`  | &check; | &check; |
| `ERROR-CODE`         | &check; | &cross; |
| `UNKNOWN-ATTRIBUTES` | &check; | &cross; |
| `REFLECTED-FROM`     | &check; | &check; |


### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `400`      | &check; | &cross; |
| `401`      | &check; | &cross; |
| `420`      | &check; | &cross; |
| `430`      | &check; | &cross; |
| `431`      | &check; | &cross; |
| `432`      | &check; | &cross; |
| `433`      | &check; | &cross; |
| `500`      | &check; | &cross; |
| `600`      | &check; | &cross; |


## [RFC 5389](https://datatracker.ietf.org/doc/html/rfc5389)

### Message types

| Type                     | Reader  | Writer  |
|:-------------------------|:-------:|:-------:|                 
| `Binding Request`        | &cross; | &cross; | 
| `Binding Response`       | &cross; | &cross; | 
| `Binding Error Response` | &cross; | &cross; | 
| `Binding Indication`     | &cross; | &cross; | 

### Header features

| Feature        | Reader  | Writer  |
|:---------------|:-------:|:-------:|
| Message length | &cross; | &cross; |
| Magic cookie   | &cross; | &cross; |
| Transaction ID | &cross; | &cross; |

### Attributes

| Attribute            | Reader  | Writer  |
|:---------------------|:-------:|:-------:|
| `MAPPED-ADDRESS`     | &cross; | &cross; |
| `XOR-MAPPED-ADDRESS` | &cross; | &cross; |
| `USERNAME`           | &cross; | &cross; |
| `MESSAGE-INTEGRITY`  | &cross; | &cross; |
| `FINGERPRINT`        | &cross; | &cross; |
| `ERROR-CODE`         | &cross; | &cross; |
| `REALM`              | &cross; | &cross; |
| `NONCE`              | &cross; | &cross; |
| `UNKNOWN-ATTRIBUTES` | &cross; | &cross; |
| `SOFTWARE`           | &cross; | &cross; |
| `ALTERNATE-SERVER`   | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `300`      | &cross; | &cross; |
| `400`      | &cross; | &cross; |
| `401`      | &cross; | &cross; |
| `420`      | &cross; | &cross; |
| `438`      | &cross; | &cross; |
| `500`      | &cross; | &cross; |


## [RFC 5245](https://datatracker.ietf.org/doc/html/rfc5245)

### Attributes

| Attribute         | Reader  | Writer  |
|:------------------|:-------:|:-------:|
| `PRIORITY`        | &cross; | &cross; |
| `USE-CANDIDATE`   | &cross; | &cross; |
| `ICE-CONTROLLED`  | &cross; | &cross; |
| `ICE-CONTROLLING` | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `487`      | &cross; | &cross; |


## [RFC 5780](https://datatracker.ietf.org/doc/html/rfc5780)

### Attributes

| Attribute         | Reader  | Writer  |
|:------------------|:-------:|:-------:|
| `CHANGE-REQUEST`  | &cross; | &cross; |
| `RESPONSE-ORIGIN` | &cross; | &cross; |
| `OTHER-ADDRESS`   | &cross; | &cross; |
| `RESPONSE-PORT`   | &cross; | &cross; |
| `PADDING`         | &cross; | &cross; |
| `CACHE-TIMEOUT`   | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `487`      | &cross; | &cross; |


## [RFC 5766](https://datatracker.ietf.org/doc/html/rfc5766)

### Message types

| Type                              | Reader  | Writer  |
|:----------------------------------|:-------:|:-------:|
| `Allocate Request`                | &cross; | &cross; |
| `Allocate Response`               | &cross; | &cross; |
| `Allocate Error Response`         | &cross; | &cross; |
| `Refresh Request`                 | &cross; | &cross; |
| `Refresh Response`                | &cross; | &cross; |
| `Refresh Error Response`          | &cross; | &cross; |
| `Send Indication`                 | &cross; | &cross; |
| `Data Indication`                 | &cross; | &cross; |
| `CreatePermission Request`        | &cross; | &cross; |
| `CreatePermission Response`       | &cross; | &cross; |
| `CreatePermission Error Response` | &cross; | &cross; |
| `ChannelBind Request`             | &cross; | &cross; |
| `ChannelBind Response`            | &cross; | &cross; |
| `ChannelBind Error Response`      | &cross; | &cross; |

### Attributes

| Attribute             | Reader  | Writer  |
|:----------------------|:-------:|:-------:|
| `CHANNEL-NUMBER`      | &cross; | &cross; |
| `LIFETIME`            | &cross; | &cross; |
| `XOR-PEER-ADDRESS`    | &cross; | &cross; |
| `DATA`                | &cross; | &cross; |
| `XOR-RELAYED-ADDRESS` | &cross; | &cross; |
| `EVEN-PORT`           | &cross; | &cross; |
| `REQUESTED-TRANSPORT` | &cross; | &cross; |
| `DONT-FRAGMENT`       | &cross; | &cross; |
| `RESERVATION-TOKEN`   | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `403`      | &cross; | &cross; |
| `437`      | &cross; | &cross; |
| `441`      | &cross; | &cross; |
| `442`      | &cross; | &cross; |
| `486`      | &cross; | &cross; |
| `508`      | &cross; | &cross; |

## [RFC 8489](https://datatracker.ietf.org/doc/html/rfc8489)

### Message types

| Type                     | Reader  | Writer  |
|:-------------------------|:-------:|:-------:|                 
| `Binding Request`        | &cross; | &cross; | 
| `Binding Response`       | &cross; | &cross; | 
| `Binding Error Response` | &cross; | &cross; | 
| `Binding Indication`     | &cross; | &cross; | 

### Header features

| Feature        | Reader  | Writer  |
|:---------------|:-------:|:-------:|
| Message length | &cross; | &cross; |
| Magic cookie   | &cross; | &cross; |
| Transaction ID | &cross; | &cross; |

### Attributes

| Attribute                  | Reader  | Writer  |
|:---------------------------|:-------:|:-------:|
| `MAPPED-ADDRESS`           | &cross; | &cross; |
| `XOR-MAPPED-ADDRESS`       | &cross; | &cross; |
| `USERNAME`                 | &cross; | &cross; |
| `USERHASH`                 | &cross; | &cross; |
| `MESSAGE-INTEGRITY`        | &cross; | &cross; |
| `MESSAGE-INTEGRITY-SHA256` | &cross; | &cross; |
| `FINGERPRINT`              | &cross; | &cross; |
| `ERROR-CODE`               | &cross; | &cross; |
| `REALM`                    | &cross; | &cross; |
| `NONCE`                    | &cross; | &cross; |
| `PASSWORD-ALGORITHMS`      | &cross; | &cross; |
| `PASSWORD-ALGORITHM`       | &cross; | &cross; |
| `UNKNOWN-ATTRIBUTES`       | &cross; | &cross; |
| `SOFTWARE`                 | &cross; | &cross; |
| `ALTERNATE-SERVER`         | &cross; | &cross; |
| `ALTERNATE-DOMAIN`         | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `300`      | &cross; | &cross; |
| `400`      | &cross; | &cross; |
| `401`      | &cross; | &cross; |
| `420`      | &cross; | &cross; |
| `438`      | &cross; | &cross; |
| `500`      | &cross; | &cross; |


## [RFC 8445](https://datatracker.ietf.org/doc/html/rfc8445)

### Attributes

| Attribute         | Reader  | Writer  |
|:------------------|:-------:|:-------:|
| `PRIORITY`        | &cross; | &cross; |
| `USE-CANDIDATE`   | &cross; | &cross; |
| `ICE-CONTROLLED`  | &cross; | &cross; |
| `ICE-CONTROLLING` | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `487`      | &cross; | &cross; |


## [RFC 8656](https://datatracker.ietf.org/doc/html/rfc8656)

### Message types

| Type                              | Reader  | Writer  |
|:----------------------------------|:-------:|:-------:|
| `Allocate Request`                | &cross; | &cross; |
| `Allocate Response`               | &cross; | &cross; |
| `Allocate Error Response`         | &cross; | &cross; |
| `Refresh Request`                 | &cross; | &cross; |
| `Refresh Response`                | &cross; | &cross; |
| `Refresh Error Response`          | &cross; | &cross; |
| `Send Indication`                 | &cross; | &cross; |
| `Data Indication`                 | &cross; | &cross; |
| `CreatePermission Request`        | &cross; | &cross; |
| `CreatePermission Response`       | &cross; | &cross; |
| `CreatePermission Error Response` | &cross; | &cross; |
| `ChannelBind Request`             | &cross; | &cross; |
| `ChannelBind Response`            | &cross; | &cross; |
| `ChannelBind Error Response`      | &cross; | &cross; |

### Attributes

| Attribute                   | Reader  | Writer  |
|:----------------------------|:-------:|:-------:|
| `CHANNEL-NUMBER`            | &cross; | &cross; |
| `LIFETIME`                  | &cross; | &cross; |
| `XOR-PEER-ADDRESS`          | &cross; | &cross; |
| `DATA`                      | &cross; | &cross; |
| `XOR-RELAYED-ADDRESS`       | &cross; | &cross; |
| `REQUESTED-ADDRESS-FAMILY`  | &cross; | &cross; |
| `EVEN-PORT`                 | &cross; | &cross; |
| `REQUESTED-TRANSPORT`       | &cross; | &cross; |
| `DONT-FRAGMENT`             | &cross; | &cross; |
| `RESERVATION-TOKEN`         | &cross; | &cross; |
| `ADDITIONAL-ADDRESS-FAMILY` | &cross; | &cross; |
| `ADDRESS-ERROR-CODE`        | &cross; | &cross; |
| `ICMP`                      | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `403`      | &cross; | &cross; |
| `437`      | &cross; | &cross; |
| `440`      | &cross; | &cross; |
| `441`      | &cross; | &cross; |
| `442`      | &cross; | &cross; |
| `443`      | &cross; | &cross; |
| `486`      | &cross; | &cross; |
| `508`      | &cross; | &cross; |

## [RFC 6679](https://datatracker.ietf.org/doc/html/rfc6679)

### Attributes

| Attribute   | Reader  | Writer  |
|:------------|:-------:|:-------:|
| `ECN-CHECK` | &cross; | &cross; |


## [RFC 7635](https://datatracker.ietf.org/doc/html/rfc7635)

### Attributes

| Attribute                   | Reader  | Writer  |
|:----------------------------|:-------:|:-------:|
| `THIRD-PARTY-AUTHORIZATION` | &cross; | &cross; |
| `ACCESS-TOKEN`              | &cross; | &cross; |


## [RFC 8016](https://datatracker.ietf.org/doc/html/rfc8016)

### Attributes

| Attribute         | Reader  | Writer  |
|:------------------|:-------:|:-------:|
| `MOBILITY-TICKET` | &cross; | &cross; |

## [RFC 6062](https://datatracker.ietf.org/doc/html/rfc6062)

### Message types

| Type                            | Reader  | Writer  |
|:--------------------------------|:-------:|:-------:|
| `Connect Request`               | &cross; | &cross; |
| `Connect Response`              | &cross; | &cross; |
| `Connect Error Response`        | &cross; | &cross; |
| `ConnectionBind Request`        | &cross; | &cross; |
| `ConnectionBind Response`       | &cross; | &cross; |
| `ConnectionBind Error Response` | &cross; | &cross; |
| `ConnectionAttempt Indication`  | &cross; | &cross; |

### Attributes

| Attribute       | Reader  | Writer  |
|:----------------|:-------:|:-------:|
| `CONNECTION-ID` | &cross; | &cross; |

### Error codes

| Error code | Reader  | Writer  |
|:-----------|:-------:|:-------:|
| `446`      | &cross; | &cross; |
| `447`      | &cross; | &cross; |