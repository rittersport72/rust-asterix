# rust-asterix
Eurocontrol ASTERIX simple encoder and decoder

## CAT34 Usage

```rust
// Encode asterix categories
let result = encode_asterix(vec);

// Decode asterix byte stream
let result = decode_asterix(bytes);
```

## CAT034 Standard User Application Profile (UAP)

|Data Item   |Description                            |Implemented |
|------------|---------------------------------------|------------|
|I034/010    |Data Source Identifier                 |&check;     | 
|I034/000    |Message Type                           |&check;     |
|I034/030    |Time of Day                            |&check;     |
|I034/020    |Sector Number                          |&check;     |
|I034/041    |Antenna Rotation Period                |&check;     |
|I034/050    |System Configuration and Status        |&cross;     |
|I034/060    |System Processing Mode                 |&cross;     |
|I034/070    |Message Count Values                   |&cross;     |
|I034/100    |Generic Polar Window                   |&cross;     |
|I034/110    |Data Filter                            |&cross;     |
|I034/120    |3D Position of Data Source             |&check;     |
|I034/090    |Colimation Error                       |&cross;     |
|REF         |Reserved Expansion Field               |&cross;     |
|SP          |Special Purpose Field                  |&cross;     |

## References
Eurocontrol ASTERIX https://www.eurocontrol.int/asterix