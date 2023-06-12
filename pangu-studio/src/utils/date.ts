import moment from 'moment'

// computed for date format
function datetimeFormat(date: string) {
  return moment(date).format('YYYY-MM-DD HH:mm:ss')
}

export default {
  datetimeFormat,
}
