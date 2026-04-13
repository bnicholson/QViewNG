import { useState, useEffect, useCallback, useRef } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import InputAdornment from '@mui/material/InputAdornment'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import ListItemText from '@mui/material/ListItemText'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import SearchIcon from '@mui/icons-material/Search'
import dayjs from 'dayjs'
import { TournamentGroupAPI } from '../features/TournamentGroupAPI'
import { TournamentAPI, type TournamentTS } from '../features/TournamentAPI'
import TournamentTable, { DEFAULT_PAGE_SIZE } from '../components/TournamentTable'
import { TournamentEditorDialog } from '../components/TournamentEditorDialog'
import { useAuth } from '../hooks/useAuth'

interface Props {
  tgid: string
  canEdit: boolean
  canCreate: boolean
}

export const TournamentGroupTournamentsPage = ({ tgid, canEdit, canCreate }: Props) => {
  const { session } = useAuth()
  const [linked, setLinked] = useState<TournamentTS[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [creatorOpen, setCreatorOpen] = useState(false)

  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE)
  const pageSizeRef = useRef(pageSize)
  pageSizeRef.current = pageSize

  // Search state
  const [query, setQuery] = useState('')
  const [searchResults, setSearchResults] = useState<TournamentTS[]>([])
  const [searching, setSearching] = useState(false)
  const [addingId, setAddingId] = useState<string | null>(null)

  const loadLinked = useCallback(async () => {
    setLoading(true)
    try {
      const raw = await TournamentGroupAPI.getTournaments(tgid, 0, 500)
      const mapped: TournamentTS[] = raw.map((t: any) => ({
        ...t,
        fromdate: t.fromdate ? dayjs(t.fromdate) : null,
        todate: t.todate ? dayjs(t.todate) : null,
      }))
      setLinked(mapped)
    } catch (e: any) {
      setError(e.message)
    } finally {
      setLoading(false)
    }
  }, [tgid])

  useEffect(() => { loadLinked() }, [loadLinked])

  const handleSearch = useCallback(async (q: string) => {
    if (!q.trim()) { setSearchResults([]); return }
    setSearching(true)
    setError(null)
    try {
      const result = await TournamentAPI.get(0, '100')
      const lower = q.toLowerCase()
      const linkedIds = new Set(linked.map(t => String(t.tid)))
      setSearchResults(
        result.items.filter(t => {
          const isSuperUser = session?.hasRole('super_user') ?? false
          return t.tname.toLowerCase().includes(lower) &&
            !linkedIds.has(String(t.tid)) &&
            (isSuperUser || t.owner_id === session?.userId)
        })
      )
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSearching(false)
    }
  }, [linked])

  useEffect(() => {
    const timeout = setTimeout(() => handleSearch(query), 300)
    return () => clearTimeout(timeout)
  }, [query, handleSearch])

  const handleAdd = async (tournament: TournamentTS) => {
    setAddingId(String(tournament.tid))
    setError(null)
    try {
      await TournamentGroupAPI.addTournament(tgid, String(tournament.tid))
      setLinked(prev => [...prev, tournament])
      setSearchResults(prev => prev.filter(t => t.tid !== tournament.tid))
    } catch (e: any) {
      setError(e.message)
    } finally {
      setAddingId(null)
    }
  }

  const handleRemove = async (tournament: TournamentTS): Promise<void> => {
    await TournamentGroupAPI.removeFromTournament(tgid, String(tournament.tid))
    setLinked(prev => prev.filter(t => t.tid !== tournament.tid))
  }

  const handleCreated = async (tournament: TournamentTS) => {
    setCreatorOpen(false)
    try {
      await TournamentGroupAPI.addTournament(tgid, String(tournament.tid))
      setLinked(prev => [...prev, tournament])
    } catch (e: any) {
      setError(e.message)
    }
  }

  const handlePageChange = (newPage: number) => {
    setPage(newPage)
  }

  const handlePageSizeChange = (newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize)
      setLinked(prev => prev.slice(0, newSize))
    } else {
      setPage(0)
      setPageSize(newSize)
    }
  }

  // Slice for client-side pagination (all linked tournaments are loaded upfront)
  const pagedRows = linked.slice(page * pageSize, page * pageSize + pageSize)

  return (
    <Box>
      <Typography variant="subtitle1" sx={{ fontWeight: 600, mb: 1 }}>
        Tournaments
      </Typography>
      <Divider sx={{ mb: 2 }} />

      {error && <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>}

      {loading ? (
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, py: 2 }}>
          <CircularProgress size={18} />
          <Typography color="text.secondary">Loading…</Typography>
        </Box>
      ) : (
        <TournamentTable
          tournaments={pagedRows}
          totalCount={linked.length}
          page={page}
          pageSize={pageSize}
          showCreateButton={canCreate}
          onCreate={() => setCreatorOpen(true)}
          showDeleteButton={false}
          showRemoveButton={canEdit}
          onDelete={async () => {}}
          onRemove={handleRemove}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      )}

      {/* Search to add */}
      {canEdit && (
        <Box sx={{ mt: 3 }}>
          <Typography variant="subtitle2" sx={{ fontWeight: 600, mb: 1, textAlign: "left" }}>
            Add Tournament to Group
          </Typography>
          <Box sx={{ display: 'flex', gap: 1, mb: 1 }}>
            <TextField
              size="small"
              placeholder="Search by tournament name…"
              value={query}
              onChange={e => setQuery(e.target.value)}
              sx={{ flex: 1, maxWidth: 420 }}
              InputProps={{
                endAdornment: (
                  <InputAdornment position="end">
                    {searching ? <CircularProgress size={14} /> : <SearchIcon fontSize="small" />}
                  </InputAdornment>
                ),
              }}
            />
          </Box>

          {searchResults.length > 0 && (
            <Paper variant="outlined" sx={{ maxWidth: 520 }}>
              <List dense disablePadding>
                {searchResults.map((t, i) => (
                  <ListItem
                    key={String(t.tid)}
                    divider={i < searchResults.length - 1}
                    secondaryAction={
                      <Button
                        size="small"
                        variant="contained"
                        disabled={addingId === String(t.tid)}
                        onClick={() => handleAdd(t)}
                        startIcon={addingId === String(t.tid) ? <CircularProgress size={12} /> : undefined}
                      >
                        Add
                      </Button>
                    }
                  >
                    <ListItemText
                      primary={t.tname}
                      secondary={[t.city, t.region, t.country].filter(Boolean).join(', ') || undefined}
                    />
                  </ListItem>
                ))}
              </List>
            </Paper>
          )}

          {!searching && query.trim() && searchResults.length === 0 && (
            <Typography variant="body2" color="text.secondary">
              No unlinked tournaments that you manage match "{query}".
            </Typography>
          )}
        </Box>
      )}

      <TournamentEditorDialog
        isOpen={creatorOpen}
        onCancel={() => setCreatorOpen(false)}
        onSave={handleCreated}
      />
    </Box>
  )
}
