interfaces {
    ethernet eth0 {
        address 192.168.1.1/24
        description "Management Interface"
    }
    loopback lo {
    }
}
service {
    ssh {
        port 22
    }
    https {
        listen-address 0.0.0.0
        listen-port 443
    }
    api {
        keys {
            id test-key {
                key test-api-key
            }
        }
    }
}
system {
    host-name vyos-test
    login {
        user vyos {
            authentication {
                encrypted-password $6$rounds=65600$pRm8IU5zjY5r53J8$kDuqXS5eSvpoPvRSKQnBuA/JIxOHSK0/WbpLF28qRgPWKNQFcFLR5z7Jf/byVPj98qmGyXYaHh9YuQMf/Oht91
                plaintext-password "vyos"
            }
        }
    }
    name-server 8.8.8.8
    ntp {
        server 0.pool.ntp.org {
        }
        server 1.pool.ntp.org {
        }
    }
    syslog {
        global {
            facility all {
                level notice
            }
            facility protocols {
                level debug
            }
        }
    }
}