import dayjs from 'dayjs'

export interface TournamentStatus {
  label: 'Coming Up' | 'In Progress' | 'Ended'
  /** Class name for TournamentCardContent's stylesheet. */
  className: string
  /** The same colour, for components styled in JS. */
  color: string
}

/** Where a tournament stands relative to today, judged by whole days. */
export const getTournamentStatus = (fromdate: dayjs.ConfigType, todate: dayjs.ConfigType): TournamentStatus => {
  const today = dayjs().startOf('day')
  const from = dayjs(fromdate).startOf('day')
  const to = dayjs(todate).startOf('day')
  if (today.isBefore(from)) return { label: 'Coming Up', className: 'statusComingUp', color: '#b8860b' }
  if (today.isAfter(to)) return { label: 'Ended', className: 'statusEnded', color: '#1565c0' }
  return { label: 'In Progress', className: 'statusInProgress', color: '#2e7d32' }
}
