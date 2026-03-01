import mitt, {Emitter} from "mitt";
import {App, InjectionKey, type Plugin} from "vue";

export type BaseEvent<T extends string, D extends any> = {
    event: T,
    data: D
};

class EmitterWrapper {
    mit: Emitter<BaseEvent<any, any>>

    constructor() {
        this.mit = mitt<BaseEvent<any, any>>();
    }

    addEventListener<T extends BaseEvent<any, any>>(n: T["event"], handler: (evt: T) => void): void {
        this.mit.on(n, handler);
    }

    emit<T extends BaseEvent<any, any>>(event: T) {
        this.mit.emit(event.event, event);
    }
}

export const EventBusKey = Symbol() as InjectionKey<EmitterWrapper>;

export default {
    install: (app: App) => {
        app.provide(EventBusKey, new EmitterWrapper());
    }
} satisfies Plugin