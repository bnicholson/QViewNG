import { useState, useEffect } from 'react'
import { TeamAPI, type TeamTS } from '../features/TeamAPI'
import { UserAPI, type UserTS } from '../features/UserAPI'
import QuizzersTable from '../components/QuizzersTable'

interface Props {
  did: string
}

export const DivisionProfileQuizzersPage = ({ did }: Props) => {
  const [quizzers, setQuizzers] = useState<UserTS[] | undefined>(undefined)

  useEffect(() => {
    Promise.all([
      TeamAPI.getByDivision(did, 0, 500),
      UserAPI.get(0, 500),
    ]).then(([teams, userResult]) => {
      const quizzerIds = new Set(
        teams.flatMap((t: TeamTS) => [
          t.quizzer_one_id, t.quizzer_two_id, t.quizzer_three_id,
          t.quizzer_four_id, t.quizzer_five_id, t.quizzer_six_id,
        ].filter((id): id is string => id !== null && id !== undefined))
      )
      setQuizzers(userResult.items.filter(u => quizzerIds.has(u.id)))
    }).catch(() => console.error('Failed to load division quizzers'))
  }, [did])

  return <QuizzersTable externalRows={quizzers} />
}
