import {State} from './types.ts'
import {Context} from 'oak/mod.ts'

export async function errorMiddleware(ctx:Context<State>, next: () => Promise<unknown>) {
    try {
        await next()
    } catch (e) {
        ctx.response.status = e.status || 400
        ctx.response.body = {
            "message": e.message,
            
        }
    }
}