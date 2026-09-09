import React, { useCallback, useEffect, useState } from 'react'
import { Link } from 'react-router-dom'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import Dialog from '@mui/material/Dialog'
import IconButton from '@mui/material/IconButton'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { UserAPI, type UserTS } from '../features/UserAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface Props {
  isOpen: boolean;
  title: string;
  /** User IDs already in the target (hidden from the list) */
  excludeIds: string[];
  onCancel: VoidFunction;
  onPick: (user: UserTS) => Promise<void>;
  /** When provided, only these users are shown instead of fetching all users from the API. */
  availableUsers?: UserTS[];
  /**
   * When set, no results are shown until the search term reaches this many characters. Forces the
   * user to search rather than browse the full list up front.
   */
  minSearchChars?: number;
  /** Optional instructional note shown above the search bar. */
  note?: string;
}

export const UserPickerDialog = ({ isOpen, title, excludeIds, onCancel, onPick, availableUsers, minSearchChars, note }: Props) => {
  const [fetchedUsers, setFetchedUsers] = useState<UserTS[]>([]);
  const [filter, setFilter] = useState('');
  const [adding, setAdding] = useState<string | null>(null);
  const [error, setError] = useState('');

  // When minSearchChars is set we defer the fetch until the user has typed enough characters,
  // rather than loading the whole user list up front.
  const deferred = minSearchChars !== undefined && !availableUsers;
  const [loading, setLoading] = useState(false);
  const [hasFetched, setHasFetched] = useState(false);

  const loadUsers = useCallback(async () => {
    if (availableUsers) return; // Skip fetch when caller provides the list
    setLoading(true);
    try {
      const result = await UserAPI.get(0, 500);
      setFetchedUsers(result.items);
      setHasFetched(true);
    } catch {
      setError('Failed to load users.');
    } finally {
      setLoading(false);
    }
  }, [availableUsers]);

  // Reset per-open state, and eagerly load only when not deferring.
  useEffect(() => {
    if (isOpen) {
      setFilter('');
      setError('');
      setAdding(null);
      setFetchedUsers([]);
      setHasFetched(false);
      if (!deferred) loadUsers();
    }
  }, [isOpen, deferred, loadUsers]);

  // Deferred mode: fetch once the search term first reaches the threshold.
  useEffect(() => {
    if (isOpen && deferred && !hasFetched && !loading
        && filter.trim().length >= (minSearchChars ?? 0)) {
      loadUsers();
    }
  }, [isOpen, deferred, hasFetched, loading, filter, minSearchChars, loadUsers]);

  const allUsers = availableUsers ?? fetchedUsers;
  const excludeSet = new Set(excludeIds);
  const lowerFilter = filter.toLowerCase();
  // When minSearchChars is set, force a search: show nothing until the term is long enough.
  const searchGated = minSearchChars !== undefined && filter.trim().length < minSearchChars;
  // Only truly loading while fetching from the API — a caller-supplied list (availableUsers) is a
  // synchronous cache lookup, so an empty one is "no results", never a perpetual "Loading…".
  const showLoading = loading || (!availableUsers && !hasFetched);
  const filtered = searchGated ? [] : allUsers.filter(u => {
    if (excludeSet.has(u.id)) return false;
    if (!lowerFilter) return true;
    const fullName = `${u.fname} ${u.mname} ${u.lname}`.toLowerCase();
    return fullName.includes(lowerFilter)
      || u.username.toLowerCase().includes(lowerFilter)
      || u.email.toLowerCase().includes(lowerFilter);
  });

  const handlePick = async (user: UserTS) => {
    setAdding(user.id);
    setError('');
    try {
      await onPick(user);
    } catch (err: any) {
      setError(err.message ?? 'Failed to add user.');
    } finally {
      setAdding(null);
    }
  };

  return (
    <Dialog fullScreen open={isOpen} onClose={onCancel} slots={{ transition: Transition }}>
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={onCancel} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            {title}
          </Typography>
        </Toolbar>
      </AppBar>

      <Box sx={{ maxWidth: 900, mx: 'auto', width: '100%', p: 2 }}>
        <Collapse in={!!error}>
          <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError('')}>
            {error}
          </Alert>
        </Collapse>

        {note && (
          <Typography variant="body2" color="text.secondary" sx={{ mb: 1.5 }}>
            {note}
          </Typography>
        )}

        <TextField
          placeholder="Search by name, username, or email..."
          value={filter}
          onChange={e => setFilter(e.target.value)}
          fullWidth
          variant="outlined"
          size="small"
          sx={{ mb: 2 }}
          autoFocus
        />

        <Box sx={{ overflowY: 'auto', maxHeight: 'calc(100vh - 180px)', borderRadius: 2, border: '1px solid #e5e7eb' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
            <thead>
              <tr style={{ background: '#f9fafb', borderBottom: '1px solid #e5e7eb', position: 'sticky', top: 0, zIndex: 1 }}>
                <th style={{ padding: '8px 14px', textAlign: 'left', fontWeight: 600, fontSize: 12, color: '#6b7280', textTransform: 'uppercase' }}>Name</th>
                <th style={{ padding: '8px 14px', textAlign: 'left', fontWeight: 600, fontSize: 12, color: '#6b7280', textTransform: 'uppercase' }}>Username</th>
                <th style={{ padding: '8px 14px', width: 80 }} />
              </tr>
            </thead>
            <tbody>
              {filtered.length === 0 ? (
                <tr>
                  <td colSpan={3} style={{ padding: '32px 14px', textAlign: 'center', color: '#9ca3af' }}>
                    {searchGated
                      ? `Type at least ${minSearchChars} characters to search.`
                      : showLoading ? 'Loading...' : 'No matching users found.'}
                  </td>
                </tr>
              ) : (
                filtered.map((u, i) => (
                  <tr
                    key={u.id}
                    style={{
                      background: i % 2 === 0 ? '#fff' : '#fafafa',
                      borderBottom: '1px solid #f3f4f6',
                    }}
                  >
                    <td style={{ padding: '8px 14px', fontWeight: 500 }}>
                      <Link to={`/user/${u.id}/overview`} target="_blank" rel="noopener noreferrer" style={{ color: '#2563eb', textDecoration: 'none' }}>
                        {`${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`}
                      </Link>
                    </td>
                    <td style={{ padding: '8px 14px', color: '#6b7280' }}>{u.username}</td>
                    <td style={{ padding: '8px 14px' }}>
                      <Button
                        size="small"
                        variant="contained"
                        disabled={adding === u.id}
                        onClick={() => handlePick(u)}
                        sx={{ textTransform: 'none', fontSize: 12, minWidth: 60 }}
                      >
                        {adding === u.id ? 'Adding...' : 'Add'}
                      </Button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </Box>

        <Typography variant="body2" color="text.secondary" sx={{ mt: 1, textAlign: 'right' }}>
          {searchGated
            ? `Enter ${minSearchChars}+ characters to search`
            : `${filtered.length} user${filtered.length !== 1 ? 's' : ''} shown`}
        </Typography>
      </Box>
    </Dialog>
  );
};
