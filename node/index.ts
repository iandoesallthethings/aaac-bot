import { getEvents } from './Calendar.js'

const date = new Date()
const calendar = `https://flagpole.com/?post_type=tribe_events&tribe-bar-date=${date}&ical=1`
const events = await getEvents(calendar)
const artEvents = events.filter((event) => event.categories.includes('ART'))

//api.evvnt.com/#publisher_list_published_events
// red and black publisher ID 8076
// curl -X GET -H "Accept: application/json" -u API_KEY:API_SECRET https://api.evvnt.com/publishers/8076/published_events/

https: console.debug(events)
