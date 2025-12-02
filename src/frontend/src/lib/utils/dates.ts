export function formatItalianDate(isoTimestamp: string): string {
	const date = new Date(isoTimestamp);

	console.log(date);

	const options: (typeof Intl.DateTimeFormat.arguments)[1] = {
		weekday: 'long',
		day: 'numeric',
		month: 'numeric',
		year: 'numeric'
	};

	return new Intl.DateTimeFormat('it-IT', options).format(date);
}
