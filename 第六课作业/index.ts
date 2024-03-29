import { ApiPromise, WsProvider } from "@polkadot/api";
const WEB_SOCKET_URL = "ws://127.0.0.1:9944";
async function connect() {
    const wsProvider = new WsProvider(WEB_SOCKET_URL);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}
async function getEvents(api: ApiPromise) {
    // Subscribe to system events via storage
    api.query.system.events((events: any[]) => {
        console.log(`\nReceived ${events.length} events:`);

        // Loop through the Vec<EventRecord>
        events.forEach((record: { event: any }) => {
            // Extract the phase, event and the event types
            const { event } = record;
            console.log(`\t\tindex: ${event.index}`);
            console.log(`\t\tdata: ${event.data}`);
        });
    });
}

async function getState(api: ApiPromise) {
    const something = await api.query.templateModule.something();
    console.log(`\tsomething: ${something}`);
    return something;
}

async function main() {
    const api = await connect();
    getState(api);
    getEvents(api);
}

main().catch((e) => {
    console.error("Something went wrong", e);
});