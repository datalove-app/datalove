(function(){
    var Remote, remote, rootAddr, rootSec, tx_blob;
    Remote = ripple.Remote;
    remote = new Remote({
        trusted: true,
        local_signing: true,
        local_fee: true,
        fee_cushion: 1.5,
        max_fee: 10,
        servers: [ {
            host: "127.0.0.1",
            port: 6006,
            secure: false
        } ]
    });
    rootAddr = "rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh";
    rootSec = "snoPBrXtMeMyMHUVTgbuqAfg1SUTb";
    remote.connect();
    function sign_share_tx(secret, senderAddr, destAddr, amount) {
        var tx_JSON, tx, unsigned;
        tx_JSON = {
            "TransactionType": "TrustSet",
            "Account": senderAddr,
            "LimitAmount": {
                "currency": "WFI",
                "value": amount,
                "issuer": destAddr
            }
        };
        tx = new ripple.Transaction();
        tx.tx_json = tx_JSON;
        tx._secret = secret;
        tx.complete();
        unsigned = tx.serialize().to_hex();
        tx.sign();
        return tx.serialize().to_hex();
    }
    tx_blob = sign_share_tx(rootSec, rootAddr, "r3kmLJN5D28dHuH8vZNUZpMC43pEHpaocV", 100);
    console.log(tx_blob);
})();