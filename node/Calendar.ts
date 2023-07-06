import ical, { DateWithTimeZone } from 'node-ical'

export async function getEvents(url: string) {
	const response = await fetch(url)
	const icalString = await response.text()
	const icalJson = ical.sync.parseICS(icalString)

	const events = Object.values(icalJson).filter(
		(item: IcalEvent) => item.type === 'VEVENT'
	) as IcalEvent[]

	return events
}

interface IcalEvent {
	type: 'VEVENT'
	params: []
	start: Date
	datetype: 'date-time'
	end: DateWithTimeZone
	dtstamp: Date
	created: DateWithTimeZone
	lastmodified: DateWithTimeZone
	uid: string
	summary: string
	description: string
	url: string
	location: String
	categories: string[]
	method: 'PUBLISH'
}
