import React, { useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import Checkbox from '@mui/material/Checkbox'
import CloseIcon from '@mui/icons-material/Close'
import Dialog from '@mui/material/Dialog'
import Divider from '@mui/material/Divider'
import IconButton from '@mui/material/IconButton'
import InputAdornment from '@mui/material/InputAdornment'
import List from '@mui/material/List'
import ListItem from '@mui/material/ListItem'
import ListItemButton from '@mui/material/ListItemButton'
import ListItemIcon from '@mui/material/ListItemIcon'
import ListItemText from '@mui/material/ListItemText'
import SearchIcon from '@mui/icons-material/Search'
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
  onCancel: VoidFunction;
  onConfirm: (selected: UserTS[]) => void;
  /** Number of open roster slots available. */
  maxSelectable: number;
  /** IDs of users already on the team — they are hidden from the list. */
  assignedIds: string[];
}

export const QuizzerPickerDialog = (props: Props) => {
  const { isOpen, onCancel, onConfirm, maxSelectable, assignedIds } = props;
  const [users, setUsers] = useState<UserTS[]>([]);
  const [search, setSearch] = useState('');
  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!isOpen) return;
    setSearch('');
    setSelected(new Set());
    UserAPI.get(0, 200)
      .then(result => setUsers(result.items))
      .catch(() => console.error('Failed to load users for quizzer picker'));
  }, [isOpen]);

  const filtered = users.filter(u => {
    if (assignedIds.includes(u.id)) return false;
    if (!search.trim()) return true;
    const q = search.toLowerCase();
    return (
      u.username.toLowerCase().includes(q) ||
      u.fname.toLowerCase().includes(q) ||
      u.lname.toLowerCase().includes(q) ||
      u.email.toLowerCase().includes(q)
    );
  });

  const toggle = (id: string) => {
    setSelected(prev => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else if (next.size < maxSelectable) {
        next.add(id);
      }
      return next;
    });
  };

  const handleConfirm = () => {
    onConfirm(users.filter(u => selected.has(u.id)));
  };

  const slotsLabel = maxSelectable === 1 ? '1 open slot' : `${maxSelectable} open slots`;

  return (
    <Dialog
      fullScreen
      open={isOpen}
      onClose={onCancel}
      slots={{ transition: Transition }}
    >
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={onCancel} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            Add Quizzers
          </Typography>
          <Typography variant="body2" sx={{ mr: 2, opacity: 0.85 }}>
            {selected.size} / {maxSelectable} selected
          </Typography>
          <Button
            autoFocus
            color="inherit"
            onClick={handleConfirm}
            disabled={selected.size === 0}
          >
            Add{selected.size > 0 ? ` (${selected.size})` : ''}
          </Button>
        </Toolbar>
      </AppBar>

      <Box sx={{ p: 2, pb: 1 }}>
        {maxSelectable === 0 ? (
          <Typography variant="body2" color="text.secondary">
            No open roster slots — remove a quizzer first.
          </Typography>
        ) : (
          <>
            <TextField
              fullWidth
              placeholder="Search by name, username, or email…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              slotProps={{
                input: {
                  startAdornment: (
                    <InputAdornment position="start">
                      <SearchIcon fontSize="small" />
                    </InputAdornment>
                  ),
                },
              }}
              sx={{ mb: 1 }}
            />
            <Typography variant="body2" color="text.secondary">
              {slotsLabel} available — select up to {maxSelectable} quizzer{maxSelectable !== 1 ? 's' : ''}.
            </Typography>
          </>
        )}
      </Box>

      <Divider />

      <List disablePadding>
        {filtered.length === 0 && (
          <ListItem>
            <ListItemText
              primary={search ? 'No users match your search.' : 'All users are already on this team.'}
              sx={{ color: 'text.secondary' }}
            />
          </ListItem>
        )}
        {filtered.map(u => {
          const isSelected = selected.has(u.id);
          const isDisabled = !isSelected && selected.size >= maxSelectable;
          const fullName = [u.fname, u.mname, u.lname].filter(Boolean).join(' ');
          return (
            <ListItem key={u.id} disablePadding divider>
              <ListItemButton onClick={() => toggle(u.id)} disabled={isDisabled}>
                <ListItemIcon>
                  <Checkbox
                    checked={isSelected}
                    tabIndex={-1}
                    disableRipple
                    disabled={isDisabled}
                  />
                </ListItemIcon>
                <ListItemText
                  primary={
                    <span>
                      {fullName}{' '}
                      <span style={{ color: '#1d9bf0', fontWeight: 500 }}>
                        @{u.username}
                      </span>
                    </span>
                  }
                  secondary={u.email}
                />
              </ListItemButton>
            </ListItem>
          );
        })}
      </List>
    </Dialog>
  );
};
