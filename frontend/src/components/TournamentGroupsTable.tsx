import { useState, useCallback, useEffect, useRef } from 'react'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CircularProgress from '@mui/material/CircularProgress'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import InputAdornment from '@mui/material/InputAdornment'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import ListItemText from '@mui/material/ListItemText'
import Paper from '@mui/material/Paper'
import TextField from '@mui/material/TextField'
import Typography from '@mui/material/Typography'
import EditIcon from '@mui/icons-material/Edit'
import SearchIcon from '@mui/icons-material/Search'
import { Link } from 'react-router-dom'
import { DataTableTemplate, DEFAULT_PAGE_SIZE, type ColumnDef } from './DataTableTemplate'
import { TournamentGroupAPI, type TournamentGroupTS } from '../features/TournamentGroupAPI'
import { TournamentGroupEditorDialog } from './TournamentGroupEditorDialog'
import { useAuth } from '../hooks/useAuth'

function formatDate(iso: string | null | undefined): string {
  if (!iso) return '—';
  return new Date(iso).toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
}

function RemoveButton({ onRemove }: { onRemove: () => Promise<void> }) {
  const [confirming, setConfirming] = useState(false)
  const [removing, setRemoving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleClick = async () => {
    if (!confirming) { setConfirming(true); setError(null); return }
    setRemoving(true)
    try {
      await onRemove()
    } catch (err: any) {
      setError(err?.message ?? 'Remove failed')
      setConfirming(false)
    } finally {
      setRemoving(false)
    }
  }

  return (
    <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
      <button
        onClick={handleClick}
        disabled={removing}
        style={{
          padding: '3px 10px',
          borderRadius: 5,
          border: `1px solid ${confirming ? '#c0392b' : '#e0e0e0'}`,
          background: confirming ? '#c0392b' : 'transparent',
          color: confirming ? '#fff' : '#c0392b',
          fontSize: 12,
          fontWeight: 600,
          cursor: removing ? 'not-allowed' : 'pointer',
          opacity: removing ? 0.6 : 1,
          transition: 'all .15s',
          whiteSpace: 'nowrap',
        }}
      >
        {removing ? 'Removing…' : confirming ? 'Confirm' : 'Remove'}
      </button>
      {confirming && !removing && (
        <button
          onClick={() => { setConfirming(false); setError(null) }}
          style={{
            padding: '3px 8px',
            borderRadius: 5,
            border: '1px solid #e0e0e0',
            background: 'transparent',
            color: '#555',
            fontSize: 12,
            cursor: 'pointer',
          }}
        >
          Cancel
        </button>
      )}
      {error && <span style={{ fontSize: 12, color: '#c0392b' }}>{error}</span>}
    </div>
  )
}

function groupColumns(
  onEdit: (group: TournamentGroupTS) => void,
  showRemoveButton: boolean,
  onRemove: (group: TournamentGroupTS) => Promise<void>,
): ColumnDef<TournamentGroupTS>[] {
  return [
    {
      header: 'Name',
      render: g => (
        <Link
          to={`/tournament-group/${g.tgid}/overview`}
          style={{ color: '#2563eb', textDecoration: 'none', fontWeight: 500, whiteSpace: 'nowrap' }}
          onMouseEnter={e => (e.currentTarget.style.textDecoration = 'underline')}
          onMouseLeave={e => (e.currentTarget.style.textDecoration = 'none')}
        >
          {g.name}
        </Link>
      ),
    },
    {
      header: 'Description',
      render: g => g.description || <span style={{ color: '#9ca3af' }}>—</span>,
    },
    {
      header: 'Created',
      render: g => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.created_at)}</span>,
    },
    {
      header: 'Last Modified',
      render: g => <span style={{ whiteSpace: 'nowrap', color: '#6b7280' }}>{formatDate(g.updated_at)}</span>,
    },
    {
      header: '',
      render: g => (
        <Box sx={{ display: 'flex', gap: 0.5, alignItems: 'center' }}>
          <IconButton size="small" onClick={() => onEdit(g)} aria-label="Edit tournament group">
            <EditIcon fontSize="small" />
          </IconButton>
          {showRemoveButton && <RemoveButton onRemove={() => onRemove(g)} />}
        </Box>
      ),
    },
  ];
}

interface Props {
  tid: string;
  showCreateButton?: boolean;
  showDeleteButton?: boolean;
}

export default function TournamentGroupsTable({ tid, showCreateButton = true, showDeleteButton = true }: Props) {
  const { session } = useAuth();
  const [groups, setGroups] = useState<TournamentGroupTS[]>([]);
  const [totalCount, setTotalCount] = useState(0);
  const [page, setPage] = useState(0);
  const [pageSize, setPageSize] = useState(DEFAULT_PAGE_SIZE);
  const [editorIsOpen, setEditorIsOpen] = useState(false);
  const [editingGroup, setEditingGroup] = useState<TournamentGroupTS | undefined>(undefined);
  const pageSizeRef = useRef(pageSize);
  pageSizeRef.current = pageSize;

  // Search/add state
  const [query, setQuery] = useState('')
  const [searchResults, setSearchResults] = useState<TournamentGroupTS[]>([])
  const [searching, setSearching] = useState(false)
  const [addingId, setAddingId] = useState<string | null>(null)
  const [error, setError] = useState<string | null>(null)

  const loadGroups = useCallback((p: number, ps: number) => {
    TournamentGroupAPI.getByTournament(tid, p, ps)
      .then(result => {
        setPage(p);
        setPageSize(ps);
        setTotalCount(result.length < ps ? p * ps + result.length : (p + 2) * ps);
        setGroups(result);
      })
      .catch(() => console.error('Failed to load tournament groups'));
  }, [tid]);

  useEffect(() => {
    loadGroups(0, pageSizeRef.current);
  }, [tid]);

  const handlePageChange = useCallback((newPage: number) => {
    loadGroups(newPage, pageSize);
  }, [pageSize, loadGroups]);

  const handlePageSizeChange = useCallback((newSize: number) => {
    if (newSize < pageSize && page === 0) {
      setPageSize(newSize);
      setGroups(prev => prev.slice(0, newSize));
    } else {
      loadGroups(0, newSize);
    }
  }, [pageSize, page, loadGroups]);

  const handleRemove = useCallback(async (row: TournamentGroupTS): Promise<void> => {
    await TournamentGroupAPI.removeFromTournament(row.tgid, tid);
    setGroups(prev => prev.filter(g => g.tgid !== row.tgid));
    setTotalCount(prev => prev - 1);
  }, [tid]);

  const openCreate = () => {
    setEditingGroup(undefined);
    setEditorIsOpen(true);
  };

  const openEdit = (group: TournamentGroupTS) => {
    setEditingGroup(group);
    setEditorIsOpen(true);
  };

  const handleSave = useCallback((saved: TournamentGroupTS) => {
    setEditorIsOpen(false);
    if (editingGroup) {
      setGroups(prev => prev.map(g => g.tgid === saved.tgid ? saved : g));
    } else {
      loadGroups(page, pageSize);
    }
  }, [editingGroup, loadGroups, page, pageSize]);

  // Search for unlinked groups as the user types
  const handleSearch = useCallback(async (q: string) => {
    if (!q.trim()) { setSearchResults([]); return }
    setSearching(true)
    setError(null)
    try {
      const result = await TournamentGroupAPI.getAll(0, 500)
      const lower = q.toLowerCase()
      const linkedIds = new Set(groups.map(g => g.tgid))
      const isSuperUser = session?.hasRole('super_user') ?? false
      setSearchResults(
        result.items.filter(g =>
          g.name.toLowerCase().includes(lower) &&
          !linkedIds.has(g.tgid) &&
          (isSuperUser || g.owner_id === session?.userId)
        )
      )
    } catch (e: any) {
      setError(e.message)
    } finally {
      setSearching(false)
    }
  }, [groups])

  useEffect(() => {
    const timeout = setTimeout(() => handleSearch(query), 300)
    return () => clearTimeout(timeout)
  }, [query, handleSearch])

  const handleAdd = async (group: TournamentGroupTS) => {
    setAddingId(group.tgid)
    setError(null)
    try {
      await TournamentGroupAPI.addTournament(group.tgid, tid)
      setGroups(prev => [...prev, group])
      setTotalCount(prev => prev + 1)
      setSearchResults(prev => prev.filter(g => g.tgid !== group.tgid))
    } catch (e: any) {
      setError(e.message)
    } finally {
      setAddingId(null)
    }
  }

  return (
    <>
      <DataTableTemplate<TournamentGroupTS>
        key={tid}
        entityLabel="Tournament Group"
        showCreateButton={showCreateButton}
        showDeleteButton={false}
        onCreate={openCreate}
        columns={groupColumns(openEdit, showDeleteButton, handleRemove)}
        rows={groups}
        totalCount={totalCount}
        getId={g => g.tgid}
        onDelete={async () => {}}
        page={page}
        pageSize={pageSize}
        onPageChange={handlePageChange}
        onPageSizeChange={handlePageSizeChange}
      />

      {/* Search to add */}
      <Box sx={{ mt: 3 }}>
        <Typography variant="subtitle2" sx={{ fontWeight: 600, mb: 1, textAlign: "left" }}>
          Add to Existing Tournament Group
        </Typography>
        <Divider sx={{ mb: 2 }} />

        {error && <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError(null)}>{error}</Alert>}

        <Box sx={{ display: 'flex', gap: 1, mb: 1 }}>
          <TextField
            size="small"
            placeholder="Search by group name…"
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
              {searchResults.map((g, i) => (
                <ListItem
                  key={g.tgid}
                  divider={i < searchResults.length - 1}
                  secondaryAction={
                    <Button
                      size="small"
                      variant="contained"
                      disabled={addingId === g.tgid}
                      onClick={() => handleAdd(g)}
                      startIcon={addingId === g.tgid ? <CircularProgress size={12} /> : undefined}
                    >
                      Add
                    </Button>
                  }
                >
                  <ListItemText
                    primary={g.name}
                    secondary={g.description || undefined}
                  />
                </ListItem>
              ))}
            </List>
          </Paper>
        )}

        {!searching && query.trim() && searchResults.length === 0 && (
          <Typography variant="body2" color="text.secondary">
            No unlinked tournament groups that you manage match "{query}".
          </Typography>
        )}
      </Box>

      <TournamentGroupEditorDialog
        tid={tid}
        isOpen={editorIsOpen}
        onCancel={() => setEditorIsOpen(false)}
        onSave={handleSave}
        initialGroup={editingGroup}
      />
    </>
  );
}
