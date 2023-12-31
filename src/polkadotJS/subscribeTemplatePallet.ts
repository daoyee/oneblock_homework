import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import '@polkadot/api-augment';

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

const WEB_SOCKET = 'ws://127.0.0.1:9944';
const connect = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider });
    await api.isReady;
    return api;
}

// 订阅templateModule的SomethingStored事件，并打印出something中存储的新值。
const subscribeSomethingStoredEvent =  async (api: ApiPromise) => {
    api.query.system.events((events: any[]) => {
        events.forEach((record: any) => {
            const {event} = record;

            // 筛选出正确的pallet和event，然后打印出something中存储的新值。
            if (event.section === 'templateModule' && event.method === 'SomethingStored') {
                console.log("发生了事件: SomethingStored")
                const { data } = event.toJSON();
                console.log('事件的内容为： ',data);
                const storedValue = event.data[0].toString();
                console.log('更新后的值:', storedValue);
              }
        });
    });
}

const main = async () => {
    const api = await connect();
    await subscribeSomethingStoredEvent(api);

    await sleep(5000000);
}


main()
.then(() => {
    console.log('exits with success');
    process.exit(0);
})
.catch(err => {
    console.log('error is ', err);
    process.exit(1);
});
