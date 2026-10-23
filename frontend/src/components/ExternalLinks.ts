/** An outside site linked from the footer and the mobile About page. */
export interface ExternalLink {
  name: string;
  url: string;
}

/** QView's partner organizations, listed under "In Partnership With". */
export const PARTNER_LINKS: ExternalLink[] = [
  { name: 'NYI Youth Quizzing', url: 'https://nyiconnect.com/youth-quizzing/' },
  { name: 'QuizStuff.com', url: 'https://www.quizstuff.com' },
  { name: 'TeenBibleQuiz.com', url: 'https://teenbiblequiz.com' },
]

/** Other quizzing sites, listed under "Related Pages". */
export const RELATED_LINKS: ExternalLink[] = [
  { name: 'YouthQuiz.com (Set Maker)', url: 'https://youthquiz.com' },
  { name: 'The Foundry (Study Materials)', url: 'https://www.thefoundrypublishing.com/youthtop/quizzing.html' },
  { name: 'NazQuizzing.org', url: 'https://www.nazquizzing.org/' },
]

/** The outside-link sections, in display order. */
export const EXTERNAL_LINK_SECTIONS: { heading: string; links: ExternalLink[] }[] = [
  { heading: 'In Partnership With', links: PARTNER_LINKS },
  { heading: 'Related Pages', links: RELATED_LINKS },
]
