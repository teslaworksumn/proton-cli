
#![allow(dead_code)]

pub enum TestKey {
    BadPrivKeyPem,
    BadPubKeyPem,
    BadPubKeyPub,
    GoodKeyPem,
    GoodKeyPub,
    GoodKey2Pem,
    GoodKey2Pub,   
}


/// Retrieves a pre-generated rsa public or private key for testing
pub fn get_test_key(key: TestKey) -> String {
    match key {
        TestKey::BadPrivKeyPem => {
"-----BEGIN R
Some stuff removed.
MIIEpAIBAAKCAQEAw9L3r5AgkzsGPizUBMgHLAYlFr5ebMkRRIDLrlVaglv0G1gX
4cKJdH+pIfGX5ObC+ggEuJdYml3vbfgnldCD4kvhG5GZ1yIBsaDlfQ==
-----END RSA PRIVATE KEY-----
".to_string()
        },
        TestKey::BadPubKeyPem => { // Valid private key for BadPubKeyPub
"-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEAw9L3r5AgkzsGPizUBMgHLAYlFr5ebMkRRIDLrlVaglv0G1gX
YgsvWfOKN27CUOegAWW9cpOTBeUHSX9BX4/iq+3FnSGA/G8YMxKnomokLGwS3Wmo
+sXlQSPqJcZjoHhnHHZg9R2DMj09fmHqocu8Zz000To8LK8iFhws+f8Knx3uShsz
DlR15jcXvxh9YIgCtMDHx3xLasLrTIop9MTNrj3OcKy3rP+o8CoO9iLJdPj95jgr
L4mnrbKuW0RmdGqqiTVmKlyCQxwXONbs6QH4Cg3CL0W57/4Y5+hU9B0YQ7wu87N/
M9a8nHNsSDcn4YbdDHajeOMzHtApXZilZ9idVQIDAQABAoIBAG3hZUZIE19KEXi1
4spEC3IyiIE4lPdp45ySwLe0ATOAsXredVB4gjtxIq90IOkQ74GZOzKdiDQYjXw6
Qc6DiGd85hQDhsMe+289Gn9SG7YpdnRzI3SDst5Ybrx/k8uphAI+kq03wDRilKzj
+KqNDczIVWjJypYwN2o8qSgmailZvrQZaqc5AtcVeW7WNj19+GhG91EhWDuk5wOa
ADHMqGLgbtNQTJh/Ur/iFv84eXQKq51jQVyD/fNEOIMy65dOm5DfTkrIdr0FLoZ2
MXrR91UvSmzfbGrMMXosw+L+L7xoO5+ihhAd5clgRf+apCz9EhUUBTPXqjBgVh7t
0x1OccECgYEA/3h0J4oNPUvAOAVKBxT8Ol++wjIKBoQh12AEYF15RsrUe6xLSKFI
Y3YKzqstK9rIytWPcqmyy1eESQ/4vkAiN+Ks+J8nN9YuFEx87fexngRqb0U1vYQP
qQwK9NjSDyTxEspSgAfPK0BGFDq6Iab03djRbCEXdj1Md8MWE4owLV0CgYEAxDrd
7HpowsmMob0tpTQOgVLarbc/G2PQ20g5vfsPeepabYrmLNZAqukmtSqQh+SXXtME
M+xCsK393K/WK5fGRtpOWPODXBHDZYqfMWfwz9DEukNA41vfwqRIbZJcbySdBmo+
m/6u2hWWc78Dus+l1+ycMTQz80qPFEe2B0iWuFkCgYEA4FYYOfhis+bnPsp2PU3S
SDh3vPRQFXTGeujYLv8mVmIawReFbJwkgyLPTrjGi/ItzOBIfXpDhYiuo7IgzoBn
DAhDuHeZ6tBr2mtocVfHY2mhRWDTVj63uczXejdEDwRzpfqBA+jXpbVlVLJVbOO9
t2qETJ2qiqw9kBWCN1psAAkCgYBj6PUFam3cf3IzB3cuHgsknWLp/9sJvxCgFW4Q
5LtEf6X7EDWRrqTOYFds9ncQObs4dIg3R5vNCUmwC3coSuaRPKWXdKT1Q7YRBTt4
lEtwBAOmMoffOu/60lpuL48Qp7urgHzQQCQWtdccp6zmeMMf5UDBbKrmvJeM49Yy
bysOQQKBgQC9jd6SzKJARDRH79EnmAhGS/rVCl2iZViQ2DK23ElhpkcHj95cKo9u
K/eaFXWNOcUfBL6FR0YqtQ6fVhPQNjeo805RKge9uo9IAIth4ZTWdr4mxMzzwZ5X
4cKJdH+pIfGX5ObC+ggEuJdYml3vbfgnldCD4kvhG5GZ1yIBsaDlfQ==
-----END RSA PRIVATE KEY-----
".to_string()
        },
        TestKey::BadPubKeyPub => { // Made with ssh-keygen -t rsa
"ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDD0vevkCCTOwY+LNQEyAcsBiUWvl5syRFEgMuuVVqCW/QbWBdiCy9Z84o3bsJQ56ABZb1yk5MF5QdJf0Ffj+Kr7cWdIYD8bxgzEqeiaiQsbBLdaaj6xeVBI+olxmOgeGccdmD1HYMyPT1+Yeqhy7xnPTTROjwsryIWHCz5/wqfHe5KGzMOVHXmNxe/GH1giAK0wMfHfEtqwutMiin0xM2uPc5srLes/6jwKg72Isl0+P3mOCsviaetsq5bRGZ0aqqJNWYqXIJDHBc41ucpAfgKDcIvRbnv/hjn6FT0HRhDvC7zs38z1rycc2xINyfhht0MdqN44zMe0CldmKVn2J1V foo@bar
".to_string()
        },
        TestKey::GoodKeyPem => { // Made with openssh genrsa -out good_key.pem 2048
"-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEAx53amIap4iq412IBgewFR9bjFdRKao9JWoxcNc4CyzjIhRi3
ewlsUL6jZGvPJ31O9Ra7EBUKSc7GBeN8tHdLfcFUqc9p24YrR85SyVqwNHyXaGWl
p5ThLC4I4kN0MtSk02nRpEGbtDwVYS5SK4ujPmlzc1s4LtYIXDf34Hg4bVB8G6h0
8BZJI1VJz1NANRO/bXvdEFtJKifx4d9vQMZ7lDz/IWvSZxe0ff6mN4MVZb5HGwRA
bbiqkGfwVTqAHt1zAm3gZ2OmaWKgZrpVfqRHYfx5bWqxNm5AmnE9j1EnEGwQRJ7t
znFqv/9SFNGtzrIojKnbC5T2rYuHtz0JerDF0QIDAQABAoIBAQDFJ9LUnQTDngCM
sn7crJmWl9YYJGIPl1AS/9i/R6cYww/B7WSjP+5cXb4RnC2xSiGnDnlZ6/4KYhDR
lhsAP2cvYLjXM0XRjVXICS9ZMpogbQI6GVMhDnnL4ffiSk9FOSNGsBhgTHCV9lYD
cvWzjqqAJTTjNbBrCfid5KWNbhAFXh6v20BRroUO3xhf+a9XbiUpjbOHU/iGV7BI
fREI+EmNmex/gT8lYR0H9O7ZMr3utuZT/RTmXhgyx/BApfOwKslQ2cJixHjuujZi
kpUxD3Wg+WqBtqMYRhNTBBsYHXRsEtxc6DhXO8WwKaK8gsDRa0vpAIUbu8Z1RYh0
TR3r/eaVAoGBAO/qN+l6Ie5fQJOZ+4r+fMPrt+VPeNRZMH/npwcgAz35+Un0/hy1
MtHx4ajX0QqX+4X9c5cEk81TO9TGS5UoRYunjF0KFBIdTtKsZjtWm9RHO295q8v5
F5Ad/NMJEdJDGvyPQ62EXQ60So/HRh7SUQtdI7Noqxdj0ByubKhOVqITAoGBANT/
+a2TkNxjTXrqXEdOAU8Xjkt5xszH8s24PxWnvmzmeXiN6EBWUPDD3MQdD2TzetNU
FyJtLZYrQoEghqwCBHEFNmGpiwgCpSzgfGt8xe231y9IS0b2fCoFe/Mpwjnj9CdY
fe1aLf/zdP5k03VPREn9duuAFzcISR3+HhbrKtULAoGBANyhS2vWGlApnqxehhor
sLdIFeS2sgquPtDRSyRdRz3+nUdrGylxZVv3EiwA1MfmPrPZAizWU8oWCkbBWQ7B
aztkCMwjEZQbCbrqOlXFbybHnxwEC35YA1hQi4nTpGT1u2KZqYZv3FhCavNQMeb0
SP2nWMaLshOfgepuaZRZ/oerAoGAFavlr29wMbw4yeTGdsrM/0saQzukqnk9jwjM
AlwK1/GrMkTd+I3Ptm93Ksz6H/9V4+KOaAZLuRe1sIWqrh2WbhllqxIg/zOlsqc0
gYs0aDdoQXtNjW7Ou0fLkLYS+OjkkoxCY7/lJkGTCb8gdLMX859CVju2IPdK2S5d
yV8sAHkCgYEAq9vdIuLgWz1YwKmg+yUF/pa7/ibfUbZ2AnBpvl6wFQraPDjipyQM
13lLZS9Z6T/jMtbqkXzTz4xwrV2Mk+zDeVs8ledUVzj8KDfVirbbpwdRMMoR3LYw
Fum32UCtZOFRmFt35dQRtwxaA9tP0/26usWe55oQsbaxdSMG9O8KJjs=
-----END RSA PRIVATE KEY-----
".to_string()
        },
        TestKey::GoodKeyPub => { // Made with openssh rsa -in good_key.pem -pubout > good_key.pub
"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAx53amIap4iq412IBgewF
R9bjFdRKao9JWoxcNc4CyzjIhRi3ewlsUL6jZGvPJ31O9Ra7EBUKSc7GBeN8tHdL
fcFUqc9p24YrR85SyVqwNHyXaGWlp5ThLC4I4kN0MtSk02nRpEGbtDwVYS5SK4uj
Pmlzc1s4LtYIXDf34Hg4bVB8G6h08BZJI1VJz1NANRO/bXvdEFtJKifx4d9vQMZ7
lDz/IWvSZxe0ff6mN4MVZb5HGwRAbbiqkGfwVTqAHt1zAm3gZ2OmaWKgZrpVfqRH
Yfx5bWqxNm5AmnE9j1EnEGwQRJ7tznFqv/9SFNGtzrIojKnbC5T2rYuHtz0JerDF
0QIDAQAB
-----END PUBLIC KEY-----
".to_string()
        },
        TestKey::GoodKey2Pem => { // Made with openssh genrsa -out good_key2.pem 2048
"-----BEGIN RSA PRIVATE KEY-----
MIIEpQIBAAKCAQEAzZmS0oVZ/pR6q1nvOSxBB20SsDAIQD2dg6zsjGXDHUDqH4XD
PN5db5J8s8baCwCxMgQEniLd4fjCcU4pAZckn1B/OE0wWwivOjGsDEP4xAKTLYHE
aPHnwudx6e83TG8Kyg8XXMQX9KzgOuWDiYFMQ3S/t1DcTO3P1CEez/Iz6Jr2GySz
0S07TVRbz2/emt3UUXL2JSxEXwnDY2I+O/52KEVeogjXGfy/xcCvdeGhiMecQlPJ
XtzVy58hzCMWnezcxGUPHWoGs3ZNcu/Us/MU8LTe9xiXcafiwve+VZdXIae+ATrh
Loa7IYnaNcScEJqhX6cEzki9eAXHg3vhqZ5wOwIDAQABAoIBAQC2G95JrCZPcbLK
DcnwCTu8WSdhedoFFM5tMIEBxq0xPythKnkJdCisx8K+9lZxwnYxSA/TBi0kl9Oa
ytFVgEvG9WUKOIypRaL318F9x8MT0vgMKdIP5p0TLS/gNqvzfrO+z+kwmNxjM7EV
We/NnwIvCb06oVvQpCAe1AtXwz+pOeloCOmelCOJgjR7FLydf/8VhWg/nbbguFEg
iYiQ8SbjodkiCCroh2dKsAJUN+xztsmRCGxTSXcSQO+MBezZ8tLYJfj2J8EP/JXx
9CVOnDKyXolNek8PwbIeElsOSseWhk2vjeJNstgmMEA8+MUNv33ir26rWhrr8K55
ApTcPIhpAoGBAPZqCIV+XsS6qpj/YvEloT0ZwO9tz1qL/3QHDtJApDgNyGsgmN4v
GnisVPWv1YcqSl6DobhOgUzKyVwHBbNAqqMuT5wrkbBRxXSTs1TNlS8Zcj0Kexwy
r8PUKGEf+P7rUbG0oklnQcets2I9ZFK6SdfoIu7GJlOjFaaK7ebbz2JdAoGBANWZ
FScfwtMSOtkMB+bET5dSNOH9NtNrWHLAiG0K5jSlE1uKJVzysDcm77OHKJoxLLrR
0nPeSn47D6axQjMUOrqPDhDpqmfWebDUpJf7/LP3BCJt3fwv7pUY44fg3vmCVdXe
62ohsnZjIpSBvIggG+lxh7SFj+jmG69pvR6WiSN3AoGBAJEKAnAI3OEw+l9J20QT
oR9kvEJxwR4Lz1XNHbU15TM1MaWUr0YANbWXwf5G/AkuJE4TbqocU1EO/4ySeZFz
27xm8ub6YiuFRhMDKQgeZdQuRyaUt7f2QayjgQ0CE76AZjqSFv/brkVJAZAukJF9
Ix/ZfE5NBZwvHvxDRBqhgSTlAoGAbjJbEmM1bRWjnKSzUuAGu+fidpDffzWLXTBt
cexElfZwRTLBUfG27c09rx5U2S/zhrKHY9XUEsm/ql0a7SVLo0H4nMsgag/hmlQq
Umrjlakjzetmshu8d9luUYS3JHyUchFe3NNjseY1Se6g6I3AOLCktYKOmzZUJCxm
MGizecMCgYEArNBamNu8SH1sNmx9ICrfxHPyL6xC20s2lXall2QHUuO33hZm/nan
KBvqz1SdYtOY3XDAF926klem7fLXbLEo2ZGkw2VWYyJXYizGyOJbNSyTKobiBTCE
mxe4lbIRMixbvedzv15zo2HPsHbubCZEZU+EWO2auEPDauzpaaF9WQM=
-----END RSA PRIVATE KEY-----
".to_string()
        },
        TestKey::GoodKey2Pub => { // Made with openssh rsa -in good_key2.pem -pubout > good_key2.pub
"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzZmS0oVZ/pR6q1nvOSxB
B20SsDAIQD2dg6zsjGXDHUDqH4XDPN5db5J8s8baCwCxMgQEniLd4fjCcU4pAZck
n1B/OE0wWwivOjGsDEP4xAKTLYHEaPHnwudx6e83TG8Kyg8XXMQX9KzgOuWDiYFM
Q3S/t1DcTO3P1CEez/Iz6Jr2GySz0S07TVRbz2/emt3UUXL2JSxEXwnDY2I+O/52
KEVeogjXGfy/xcCvdeGhiMecQlPJXtzVy58hzCMWnezcxGUPHWoGs3ZNcu/Us/MU
8LTe9xiXcafiwve+VZdXIae+ATrhLoa7IYnaNcScEJqhX6cEzki9eAXHg3vhqZ5w
OwIDAQAB
-----END PUBLIC KEY-----
".to_string()
        },
    }


}
