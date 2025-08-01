import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../styles/AttachmentsPage.css';

interface EmailAttachment {
  id: number;
  user_id: number;
  email_account_id: number;
  filename: string;
  file_path: string;
  file_size: number;
  mime_type: string;
  category: string;
  sender_email: string;
  subject: string;
  received_at: string;
  created_at: string;
}

interface AttachmentCategory {
  category: string;
  count: number;
  total_size: number;
}

interface AttachmentsPageProps {
  token: string;
}

const AttachmentsPage: React.FC<AttachmentsPageProps> = ({ token }) => {
  const [attachments, setAttachments] = useState<EmailAttachment[]>([]);
  const [categories, setCategories] = useState<AttachmentCategory[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string>('all');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedAttachments, setSelectedAttachments] = useState<Set<number>>(new Set());
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'date' | 'size' | 'name'>('date');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('desc');

  useEffect(() => {
    loadData();
  }, [token]);

  const loadData = async () => {
    try {
      setLoading(true);
      const [attachmentsData, categoriesData] = await Promise.all([
        invoke<EmailAttachment[]>('get_attachments', { token }),
        invoke<AttachmentCategory[]>('get_attachment_categories', { token }),
      ]);
      
      setAttachments(attachmentsData);
      setCategories(categoriesData);
      setError(null);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load attachments data:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteAttachments = async (attachmentIds: number[]) => {
    try {
      await invoke('delete_attachments', {
        token,
        attachmentIds,
      });
      
      setSelectedAttachments(new Set());
      await loadData();
    } catch (err) {
      setError(err as string);
    }
  };

  const handleSelectAttachment = (attachmentId: number) => {
    const newSelected = new Set(selectedAttachments);
    if (newSelected.has(attachmentId)) {
      newSelected.delete(attachmentId);
    } else {
      newSelected.add(attachmentId);
    }
    setSelectedAttachments(newSelected);
  };

  const handleSelectAll = () => {
    if (selectedAttachments.size === filteredAttachments.length) {
      setSelectedAttachments(new Set());
    } else {
      setSelectedAttachments(new Set(filteredAttachments.map(a => a.id)));
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const formatDate = (dateString: string): string => {
    return new Date(dateString).toLocaleString();
  };

  const getCategoryIcon = (category: string): string => {
    switch (category.toLowerCase()) {
      case 'document': return '📄';
      case 'image': return '🖼️';
      case 'video': return '🎥';
      case 'audio': return '🎵';
      case 'archive': return '📦';
      case 'spreadsheet': return '📊';
      case 'presentation': return '📈';
      default: return '📎';
    }
  };

  const getCategoryColor = (category: string): string => {
    switch (category.toLowerCase()) {
      case 'document': return '#007bff';
      case 'image': return '#28a745';
      case 'video': return '#dc3545';
      case 'audio': return '#ffc107';
      case 'archive': return '#6c757d';
      case 'spreadsheet': return '#17a2b8';
      case 'presentation': return '#fd7e14';
      default: return '#6c757d';
    }
  };

  const filteredAttachments = attachments
    .filter(attachment => {
      const matchesCategory = selectedCategory === 'all' || attachment.category === selectedCategory;
      const matchesSearch = searchTerm === '' || 
        attachment.filename.toLowerCase().includes(searchTerm.toLowerCase()) ||
        attachment.sender_email.toLowerCase().includes(searchTerm.toLowerCase()) ||
        attachment.subject.toLowerCase().includes(searchTerm.toLowerCase());
      return matchesCategory && matchesSearch;
    })
    .sort((a, b) => {
      let comparison = 0;
      switch (sortBy) {
        case 'date':
          comparison = new Date(a.received_at).getTime() - new Date(b.received_at).getTime();
          break;
        case 'size':
          comparison = a.file_size - b.file_size;
          break;
        case 'name':
          comparison = a.filename.localeCompare(b.filename);
          break;
      }
      return sortOrder === 'asc' ? comparison : -comparison;
    });

  const totalSize = categories.reduce((sum, cat) => sum + cat.total_size, 0);
  const totalCount = categories.reduce((sum, cat) => sum + cat.count, 0);

  if (loading) {
    return (
      <div className="attachments-page">
        <div className="loading">Loading attachments...</div>
      </div>
    );
  }

  return (
    <div className="attachments-page">
      <div className="page-header">
        <h1>Email Attachments</h1>
        <div className="header-stats">
          <div className="stat-item">
            <span className="stat-value">{totalCount}</span>
            <span className="stat-label">Total Files</span>
          </div>
          <div className="stat-item">
            <span className="stat-value">{formatFileSize(totalSize)}</span>
            <span className="stat-label">Total Size</span>
          </div>
        </div>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      <div className="content-layout">
        <div className="categories-sidebar">
          <h2>Categories</h2>
          <div className="categories-list">
            <div 
              className={`category-item ${selectedCategory === 'all' ? 'selected' : ''}`}
              onClick={() => setSelectedCategory('all')}
            >
              <div className="category-info">
                <span className="category-icon">📁</span>
                <span className="category-name">All Files</span>
              </div>
              <div className="category-stats">
                <span className="category-count">{totalCount}</span>
                <span className="category-size">{formatFileSize(totalSize)}</span>
              </div>
            </div>
            
            {categories.map(category => (
              <div 
                key={category.category}
                className={`category-item ${selectedCategory === category.category ? 'selected' : ''}`}
                onClick={() => setSelectedCategory(category.category)}
              >
                <div className="category-info">
                  <span className="category-icon">{getCategoryIcon(category.category)}</span>
                  <span className="category-name">{category.category}</span>
                </div>
                <div className="category-stats">
                  <span className="category-count">{category.count}</span>
                  <span className="category-size">{formatFileSize(category.total_size)}</span>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="attachments-main">
          <div className="attachments-toolbar">
            <div className="search-controls">
              <input
                type="text"
                placeholder="Search attachments..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="search-input"
              />
            </div>
            
            <div className="sort-controls">
              <select
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as 'date' | 'size' | 'name')}
                className="sort-select"
              >
                <option value="date">Sort by Date</option>
                <option value="size">Sort by Size</option>
                <option value="name">Sort by Name</option>
              </select>
              
              <button
                className="sort-order-btn"
                onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
                title={`Sort ${sortOrder === 'asc' ? 'Descending' : 'Ascending'}`}
              >
                {sortOrder === 'asc' ? '↑' : '↓'}
              </button>
            </div>
            
            {selectedAttachments.size > 0 && (
              <div className="selection-actions">
                <span className="selection-count">
                  {selectedAttachments.size} selected
                </span>
                <button
                  className="btn btn-danger btn-sm"
                  onClick={() => handleDeleteAttachments(Array.from(selectedAttachments))}
                >
                  Delete Selected
                </button>
              </div>
            )}
          </div>

          {filteredAttachments.length === 0 ? (
            <div className="empty-state">
              {selectedCategory === 'all' ? (
                <>
                  <p>No attachments found.</p>
                  <p>Attachments will appear here when emails with attachments are received.</p>
                </>
              ) : (
                <>
                  <p>No {selectedCategory} attachments found.</p>
                  <p>Try selecting a different category or adjusting your search.</p>
                </>
              )}
            </div>
          ) : (
            <>
              <div className="attachments-header">
                <div className="select-all">
                  <input
                    type="checkbox"
                    checked={selectedAttachments.size === filteredAttachments.length && filteredAttachments.length > 0}
                    onChange={handleSelectAll}
                  />
                  <span>Select All ({filteredAttachments.length})</span>
                </div>
              </div>
              
              <div className="attachments-grid">
                {filteredAttachments.map(attachment => (
                  <div 
                    key={attachment.id} 
                    className={`attachment-card ${selectedAttachments.has(attachment.id) ? 'selected' : ''}`}
                  >
                    <div className="attachment-header">
                      <input
                        type="checkbox"
                        checked={selectedAttachments.has(attachment.id)}
                        onChange={() => handleSelectAttachment(attachment.id)}
                        onClick={(e) => e.stopPropagation()}
                      />
                      <div 
                        className="attachment-category"
                        style={{ backgroundColor: getCategoryColor(attachment.category) }}
                      >
                        {getCategoryIcon(attachment.category)}
                      </div>
                    </div>
                    
                    <div className="attachment-info">
                      <div className="attachment-filename" title={attachment.filename}>
                        {attachment.filename}
                      </div>
                      
                      <div className="attachment-details">
                        <div className="detail-row">
                          <span className="label">Size:</span>
                          <span className="value">{formatFileSize(attachment.file_size)}</span>
                        </div>
                        
                        <div className="detail-row">
                          <span className="label">Type:</span>
                          <span className="value">{attachment.mime_type}</span>
                        </div>
                        
                        <div className="detail-row">
                          <span className="label">From:</span>
                          <span className="value" title={attachment.sender_email}>
                            {attachment.sender_email}
                          </span>
                        </div>
                        
                        <div className="detail-row">
                          <span className="label">Subject:</span>
                          <span className="value" title={attachment.subject}>
                            {attachment.subject}
                          </span>
                        </div>
                        
                        <div className="detail-row">
                          <span className="label">Received:</span>
                          <span className="value">{formatDate(attachment.received_at)}</span>
                        </div>
                      </div>
                    </div>
                    
                    <div className="attachment-actions">
                      <button
                        className="btn btn-outline btn-sm"
                        onClick={() => {
                          // Open file location or download
                          console.log('Open file:', attachment.file_path);
                        }}
                      >
                        Open
                      </button>
                      
                      <button
                        className="btn btn-danger btn-sm"
                        onClick={() => handleDeleteAttachments([attachment.id])}
                      >
                        Delete
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
};

export default AttachmentsPage;