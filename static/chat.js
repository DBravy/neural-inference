(() => {
    const state = {
        messages: [],
        sending: false,
        profileId: null,  // Track which profile we're analyzing
        elements: {}
    };

    function qs(id) {
        return document.getElementById(id);
    }

    function setSending(sending) {
        state.sending = sending;
        if (state.elements.input) {
            state.elements.input.disabled = sending;
        }
        if (state.elements.sendBtn) {
            state.elements.sendBtn.disabled = sending;
        }
    }

    function updateSampleQuestionsVisibility() {
        const sampleQuestions = state.elements.sampleQuestions;
        if (!sampleQuestions) return;

        // Show sample questions if there are no messages and input is empty
        const shouldShow = state.messages.length === 0 && 
                          (!state.elements.input.value || state.elements.input.value.trim() === '');
        
        if (shouldShow) {
            sampleQuestions.classList.remove('hidden');
        } else {
            sampleQuestions.classList.add('hidden');
        }
    }

    function handleSampleQuestionClick(question) {
        // Set the input value
        state.elements.input.value = question;
        // Hide sample questions
        state.elements.sampleQuestions.classList.add('hidden');
        // Trigger send immediately
        sendMessage();
    }

    function appendMessage(role, content) {
        const item = document.createElement('div');
        item.className = role === 'user' ? 'chat-msg user' : 'chat-msg assistant';
        // Parse markdown for assistant messages, keep user messages as plain text
        if (role === 'assistant' && typeof marked !== 'undefined') {
            item.innerHTML = marked.parse(content);
        } else {
            item.textContent = content;
        }
        state.elements.messages.appendChild(item);
        state.elements.messages.scrollTop = state.elements.messages.scrollHeight;
    }

    function showLoadingIndicator() {
        const loadingDiv = document.createElement('div');
        loadingDiv.className = 'chat-msg loading';
        loadingDiv.id = 'chat-loading-indicator';
        loadingDiv.innerHTML = `
            <div class="typing-dots">
                <span></span>
                <span></span>
                <span></span>
            </div>
        `;
        state.elements.messages.appendChild(loadingDiv);
        state.elements.messages.scrollTop = state.elements.messages.scrollHeight;
    }

    function removeLoadingIndicator() {
        const loadingDiv = document.getElementById('chat-loading-indicator');
        if (loadingDiv) {
            loadingDiv.remove();
        }
    }

    async function sendMessage(evt) {
        if (evt) evt.preventDefault();
        if (state.sending) return;

        const text = state.elements.input.value.trim();
        if (!text) return;

        // Push user message locally
        state.messages.push({ role: 'user', content: text });
        appendMessage('user', text);
        state.elements.input.value = '';

        // Hide sample questions after first message
        updateSampleQuestionsVisibility();

        setSending(true);
        showLoadingIndicator();
        
        try {
            // Include profile_id in the request so AI has context
            const requestBody = { 
                messages: state.messages,
                profile_id: state.profileId || 'healthy'  // Default to 'healthy' if not set
            };
            
            const response = await fetch('/api/chat', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(requestBody)
            });

            removeLoadingIndicator();

            if (!response.ok) {
                const errText = await response.text();
                appendMessage('assistant', `Error: ${errText || response.statusText}`);
                return;
            }

            const data = await response.json();
            const reply = data && data.reply ? data.reply : 'No reply.';
            state.messages.push({ role: 'assistant', content: reply });
            appendMessage('assistant', reply);
        } catch (e) {
            removeLoadingIndicator();
            appendMessage('assistant', `Network error: ${e.message}`);
        } finally {
            setSending(false);
        }
    }

    window.addEventListener('DOMContentLoaded', () => {
        state.elements.messages = qs('chat-messages');
        state.elements.form = qs('chat-form');
        state.elements.input = qs('chat-input');
        state.elements.sendBtn = qs('chat-send');
        state.elements.sampleQuestions = qs('sample-questions');

        if (state.elements.form) {
            state.elements.form.addEventListener('submit', sendMessage);
        }

        // Set up input listener to hide sample questions when user types
        if (state.elements.input) {
            state.elements.input.addEventListener('input', updateSampleQuestionsVisibility);
        }

        // Set up click listeners for sample question cards
        if (state.elements.sampleQuestions) {
            const questionCards = state.elements.sampleQuestions.querySelectorAll('.sample-question');
            questionCards.forEach(card => {
                card.addEventListener('click', () => {
                    const question = card.getAttribute('data-question');
                    if (question) {
                        handleSampleQuestionClick(question);
                    }
                });
            });
        }

        // Show sample questions initially when chat opens
        updateSampleQuestionsVisibility();
    });

    // Function to clear the chat
    function clearChat() {
        // Clear the messages array
        state.messages = [];
        
        // Clear the DOM
        if (state.elements.messages) {
            state.elements.messages.innerHTML = '';
        }
        
        // Clear the input
        if (state.elements.input) {
            state.elements.input.value = '';
        }
        
        // Show sample questions again
        updateSampleQuestionsVisibility();
    }

    // Function to set the current profile ID for context
    function setProfileContext(profileId) {
        state.profileId = profileId;
        console.log('Chat context updated to profile:', profileId);
    }

    // Export functions so they can be called from the main page
    window.resetChatSampleQuestions = updateSampleQuestionsVisibility;
    window.clearChat = clearChat;
    window.setChatProfileContext = setProfileContext;
})();


