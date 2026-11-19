/**
 * ─────────────────────────────────────────────────────────────────────────────
 *               SINGLETON CHIBI NAHIDA - COMPANION STATE MACHINE
 *   Rules: ONLY ONE (1) Chibi object may exist at any time.
 * ─────────────────────────────────────────────────────────────────────────────
 */

(function () {
  'use strict';

  // Base path for assets (handles GitHub Pages repo prefix if present)
  const isGhPages = window.location.pathname.startsWith('/akashic-records');
  const BASE_ASSETS = isGhPages ? '/akashic-records/assets/chibi' : '/assets/chibi';

  const SPRITES = {
    // 01 Movement
    run: [
      `${BASE_ASSETS}/01_movement/nahida_run_01.png`,
      `${BASE_ASSETS}/01_movement/nahida_run_02.png`,
      `${BASE_ASSETS}/01_movement/nahida_run_03.png`,
      `${BASE_ASSETS}/01_movement/nahida_run_04.png`,
      `${BASE_ASSETS}/01_movement/nahida_run_05.png`,
    ],
    walkLeft: [
      `${BASE_ASSETS}/01_movement/nahida_walk_left_01.png`,
      `${BASE_ASSETS}/01_movement/nahida_walk_left_02.png`,
      `${BASE_ASSETS}/01_movement/nahida_walk_left_03.png`,
    ],
    // 02 Physics Events
    impact: `${BASE_ASSETS}/02_physics_events/nahida_impact_01.png`,
    fall: [
      `${BASE_ASSETS}/02_physics_events/nahida_fall_01.png`,
      `${BASE_ASSETS}/02_physics_events/nahida_fall_02.png`,
      `${BASE_ASSETS}/02_physics_events/nahida_fall_03.png`,
    ],
    land: `${BASE_ASSETS}/02_physics_events/nahida_land_01.png`,
    recover: `${BASE_ASSETS}/02_physics_events/nahida_recover_01.png`,
    // 03 Chat Interaction
    think: `${BASE_ASSETS}/03_chat_interaction/nahida_think_pose.png`,
    ponder: `${BASE_ASSETS}/03_chat_interaction/nahida_ponder_pose.png`,
    askInvite: `${BASE_ASSETS}/03_chat_interaction/nahida_ask_invite.png`,
    loading: `${BASE_ASSETS}/03_chat_interaction/nahida_loading_01.png`,
    excited: `${BASE_ASSETS}/03_chat_interaction/nahida_excited_01.png`,
    answering: `${BASE_ASSETS}/03_chat_interaction/nahida_answering_01.png`,
    // 04 Exit
    leave01: `${BASE_ASSETS}/04_exit/nahida_leave_01.png`,
    leave02: `${BASE_ASSETS}/04_exit/nahida_leave_02.png`,
  };

  // Preload all sprites for zero-lag 60fps transitions
  function preloadImages() {
    const urls = [
      ...SPRITES.run,
      ...SPRITES.walkLeft,
      SPRITES.impact,
      ...SPRITES.fall,
      SPRITES.land,
      SPRITES.recover,
      SPRITES.think,
      SPRITES.ponder,
      SPRITES.askInvite,
      SPRITES.loading,
      SPRITES.excited,
      SPRITES.answering,
      SPRITES.leave01,
      SPRITES.leave02,
    ];
    urls.forEach((u) => {
      const img = new Image();
      img.src = u;
    });
  }

  // Particle System (Leaves & Stars)
  function spawnParticle(x, y, type = 'leaf') {
    const p = document.createElement('div');
    p.className = 'nahida-particle';
    const isLeaf = type === 'leaf';
    p.textContent = isLeaf ? (Math.random() > 0.5 ? '🍃' : '🌿') : (Math.random() > 0.5 ? '✨' : '💫');
    p.style.cssText = `
      position: fixed;
      left: ${x + (Math.random() * 24 - 12)}px;
      top: ${y + (Math.random() * 24 - 12)}px;
      font-size: ${isLeaf ? '14px' : '12px'};
      pointer-events: none;
      z-index: 9999;
      opacity: 1;
      transform: scale(0.6) rotate(${Math.random() * 360}deg);
      transition: all 0.8s cubic-bezier(0.2, 0.8, 0.2, 1);
    `;
    document.body.appendChild(p);

    requestAnimationFrame(() => {
      p.style.transform = `translate(${(Math.random() - 0.5) * 60}px, ${(Math.random() - 0.5) * 60 - 20}px) scale(1.1) rotate(${Math.random() * 360}deg)`;
      p.style.opacity = '0';
    });

    setTimeout(() => {
      if (p.parentNode) p.parentNode.removeChild(p);
    }, 850);
  }

  // Singleton State Machine Manager
  class NahidaCompanion {
    constructor() {
      // GLOBAL SINGLETON GUARD
      const existing = document.getElementById('nahida_companion');
      if (existing) {
        existing.parentNode.removeChild(existing);
      }

      this.state = 0; // State 0: Spawning
      this.x = 20;
      this.y = 20;
      this.vx = 2.4;
      this.vy = 0;
      this.scale = 1.0;
      this.opacity = 1.0;
      this.flip = 1; // 1 = right, -1 = left
      this.frameIndex = 0;
      this.lastFrameTime = 0;
      this.stateStartTime = performance.now();
      this.inChat = false;
      this.isSubmitting = false;

      this.createDOM();
      this.initEventListeners();
      preloadImages();
      this.enterState(0);
      this.loop = this.loop.bind(this);
      requestAnimationFrame(this.loop);
    }

    createDOM() {
      this.root = document.createElement('div');
      this.root.id = 'nahida_companion';
      this.root.style.cssText = `
        position: fixed;
        left: 0px;
        top: 0px;
        z-index: 9990;
        cursor: pointer;
        user-select: none;
        transform-origin: bottom center;
        will-change: transform, left, top;
        display: flex;
        flex-direction: column;
        align-items: center;
      `;

      // Speech Bubble
      this.bubble = document.createElement('div');
      this.bubble.className = 'nahida-speech-bubble';
      this.bubble.style.cssText = `
        position: relative;
        margin-bottom: 6px;
        padding: 3px 8px;
        background: rgba(9, 9, 11, 0.92);
        border: 1px solid rgba(63, 63, 70, 0.9);
        border-radius: 12px;
        color: #f4f4f5;
        font-family: monospace;
        font-size: 11px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
        white-space: nowrap;
        pointer-events: none;
        transition: opacity 0.2s, transform 0.2s;
        display: flex;
        align-items: center;
        gap: 4px;
      `;
      this.bubble.innerHTML = `<span>💬</span><span>Ask me!</span><span style="display:inline-block;width:5px;height:5px;background:#34d399;border-radius:50%"></span>`;
      this.root.appendChild(this.bubble);

      // Sprite Image
      this.img = document.createElement('img');
      this.img.alt = 'Nahida Chibi';
      this.img.style.cssText = `
        width: 68px;
        height: auto;
        display: block;
        filter: drop-shadow(0 6px 14px rgba(0,0,0,0.6));
        pointer-events: none;
      `;
      this.img.src = SPRITES.run[0];
      this.root.appendChild(this.img);

      // Click on Nahida opens/toggles chat
      this.root.addEventListener('click', (e) => {
        e.stopPropagation();
        this.onUserClick();
      });

      document.body.appendChild(this.root);
    }

    setBubble(text, icon = '💬', dotColor = '#34d399') {
      if (!text) {
        this.bubble.style.opacity = '0';
        return;
      }
      this.bubble.style.opacity = '1';
      this.bubble.innerHTML = `<span>${icon}</span><span>${text}</span>${dotColor ? `<span style="display:inline-block;width:5px;height:5px;background:${dotColor};border-radius:50%"></span>` : ''}`;
    }

    onUserClick() {
      if (this.inChat) {
        this.closeChat();
      } else {
        this.openChat();
      }
    }

    openChat() {
      this.inChat = true;
      // Dispatch custom event for Leptos AthenaWidget
      window.dispatchEvent(new CustomEvent('nahida:open-chat'));
      this.enterState(6);
    }

    closeChat() {
      this.inChat = false;
      window.dispatchEvent(new CustomEvent('nahida:close-chat'));
      this.enterState(9); // State 9: Departure
    }

    initEventListeners() {
      // Listen to iframe / Athena events
      window.addEventListener('message', (e) => {
        if (!e.data || e.data.type !== 'ATHENA_STATE') return;
        const s = e.data.state;
        if (s === 'THINKING') {
          this.enterState(7);
        } else if (s === 'ANSWERING') {
          this.enterState(8);
        } else if (s === 'WAIT') {
          this.enterState(8.5);
        }
      });

      // Synchronize when Leptos changes chat state
      window.addEventListener('athena:external-open', () => {
        if (!this.inChat) {
          this.inChat = true;
          this.enterState(6);
        }
      });
      window.addEventListener('athena:external-close', () => {
        if (this.inChat) {
          this.inChat = false;
          this.enterState(9);
        }
      });
    }

    enterState(newState) {
      this.state = newState;
      this.stateStartTime = performance.now();
      this.frameIndex = 0;

      switch (this.state) {
        case 0: // Spawning
          this.x = 20;
          this.y = 20;
          this.opacity = 1.0;
          this.scale = 0.0;
          this.flip = 1;
          this.img.src = SPRITES.run[0];
          this.setBubble('Awakening...', '🌱', '#34d399');
          for (let i = 0; i < 7; i++) {
            spawnParticle(this.x + 30, this.y + 40, 'leaf');
          }
          break;

        case 1: // Patrol Right
          this.scale = 1.0;
          this.opacity = 1.0;
          this.flip = 1;
          this.vx = 2.4 + Math.random() * 0.8;
          this.setBubble('Ask Athena!', '💬', '#34d399');
          break;

        case 2: // Edge Contact (Float Impact)
          this.img.src = SPRITES.impact;
          this.setBubble('ouch!', '💫', '#f59e0b');
          spawnParticle(this.x + 50, this.y + 20, 'star');
          spawnParticle(this.x + 50, this.y + 50, 'star');
          break;

        case 3: // Anti-Gravity Falling
          this.vy = 0.4;
          this.img.src = SPRITES.fall[0];
          this.setBubble('whoa~', '🍃', '#10b981');
          break;

        case 4: // Recovery
          this.img.src = SPRITES.land;
          this.setBubble('bonk!', '💫', '#f59e0b');
          spawnParticle(this.x + 30, this.y + 50, 'star');
          break;

        case 5: // Random Patrol Left
          this.scale = 1.0;
          this.flip = -1;
          this.vx = -(2.0 + Math.random() * 0.7);
          this.setBubble('Exploring...', '🌿', '#10b981');
          this.nextRandomCheck = performance.now() + 3000 + Math.random() * 2000;
          break;

        case 6: // Interrupted Chat Opening (Ask Me)
          this.flip = 1;
          this.scale = 1.0;
          this.opacity = 1.0;
          this.img.src = SPRITES.think;
          this.setBubble('Ask me anything!', '✨', '#34d399');
          break;

        case 7: // Thinking & Loading
          this.img.src = SPRITES.loading;
          this.setBubble('Thinking...', '💭', '#60a5fa');
          break;

        case 8: // Excitement / Answering
          this.img.src = SPRITES.excited;
          this.setBubble('Got it!', '🌟', '#fbbf24');
          for (let i = 0; i < 5; i++) {
            spawnParticle(this.x + 30, this.y + 30, 'star');
          }
          break;

        case 8.5: // Multi-Question Wait
          this.img.src = SPRITES.ponder;
          this.setBubble('Here to help!', '✨', '#34d399');
          break;

        case 9: // Departure & Disappearing
          this.img.src = SPRITES.leave01;
          this.setBubble('See you soon!', '👋', '#34d399');
          for (let i = 0; i < 8; i++) {
            spawnParticle(this.x + 30, this.y + 40, 'leaf');
          }
          break;

        case 10: // Instant Recycle
          this.enterState(0);
          break;
      }
    }

    loop(time) {
      const dt = time - this.lastFrameTime;
      const elapsed = time - this.stateStartTime;
      const floorY = window.innerHeight - 90;
      const rightEdge = window.innerWidth - 85;

      switch (this.state) {
        case 0: // Spawning pop (0.0 to 1.0 in 250ms)
          this.scale = Math.min(1.0, elapsed / 250);
          if (elapsed >= 250) {
            this.enterState(1);
          }
          break;

        case 1: // Patrol Right
          this.x += this.vx;
          // Animate running frames (12fps ~ 83ms)
          if (dt > 83) {
            this.frameIndex = (this.frameIndex + 1) % SPRITES.run.length;
            this.img.src = SPRITES.run[this.frameIndex];
            this.lastFrameTime = time;
          }
          // Floor alignment
          this.y = floorY;

          // Random 1% chance for speed variation
          if (Math.random() < 0.01) {
            this.vx = 1.8 + Math.random() * 1.5;
          }

          // Edge hit
          if (this.x >= rightEdge) {
            this.x = rightEdge;
            this.enterState(2);
          }
          break;

        case 2: // Edge Contact (Float Impact)
          this.x = rightEdge - Math.sin(Math.min(1, elapsed / 300) * Math.PI) * 6;
          if (elapsed >= 300) {
            this.enterState(3);
          }
          break;

        case 3: // Anti-Gravity Falling (Dendro low gravity + sine sway)
          this.vy = Math.min(2.0, this.vy + 0.04); // low gravity
          this.y += this.vy;
          // Sine-wave drift on X (-10px to +10px)
          const sway = Math.sin(elapsed * 0.005) * 10;
          this.root.style.transform = `translateX(${sway}px) rotate(${Math.sin(elapsed * 0.004) * 12}deg)`;

          // Frame tumble
          const fallIdx = Math.floor(elapsed / 180) % SPRITES.fall.length;
          this.img.src = SPRITES.fall[fallIdx];

          // Occasional leaf particles during descent
          if (Math.random() < 0.06) {
            spawnParticle(this.x + 30, this.y + 40, 'leaf');
          }

          if (this.y >= floorY) {
            this.y = floorY;
            this.root.style.transform = 'none';
            this.enterState(4);
          }
          break;

        case 4: // Recovery Sequence
          if (elapsed < 500) {
            this.img.src = SPRITES.land;
          } else if (elapsed < 900) {
            this.img.src = SPRITES.recover;
          } else if (elapsed < 1300) {
            this.img.src = SPRITES.think;
          } else {
            this.enterState(5);
          }
          break;

        case 5: // Random Patrol Left
          this.y = floorY;
          this.x += this.vx;

          if (dt > 100) {
            this.frameIndex = (this.frameIndex + 1) % SPRITES.walkLeft.length;
            this.img.src = SPRITES.walkLeft[this.frameIndex];
            this.lastFrameTime = time;
          }

          // Randomized behavior checks every 3-5 seconds
          if (time > this.nextRandomCheck) {
            this.nextRandomCheck = time + 3000 + Math.random() * 2000;
            const roll = Math.floor(Math.random() * 100) + 1;
            if (roll >= 41 && roll <= 70) {
              // Pause & breathe
              this.img.src = SPRITES.ponder;
              this.vx = 0;
              setTimeout(() => {
                if (this.state === 5) this.vx = -(2.0 + Math.random() * 0.7);
              }, 1200);
            } else if (roll >= 71 && roll <= 85) {
              // Quick excited hop
              this.img.src = SPRITES.excited;
              spawnParticle(this.x + 30, this.y + 20, 'star');
              this.vx = 0;
              setTimeout(() => {
                if (this.state === 5) this.vx = -(2.0 + Math.random() * 0.7);
              }, 1000);
            } else if (roll >= 86) {
              // Change direction back to right
              this.enterState(1);
            }
          }

          if (this.x <= 15) {
            this.x = 15;
            this.enterState(1); // turn and walk right
          }
          break;

        case 6: // Interrupted Chat Opening (Docked beside chatbox)
          // Anchor point: bottom-right next to chat modal
          const targetX = Math.max(20, window.innerWidth - 490);
          const targetY = window.innerHeight - 95;
          this.x += (targetX - this.x) * 0.18;
          this.y += (targetY - this.y) * 0.18;

          if (elapsed > 300) {
            this.img.src = SPRITES.askInvite;
          }
          break;

        case 7: // Thinking & Loading
          const targetX7 = Math.max(20, window.innerWidth - 490);
          const targetY7 = window.innerHeight - 95;
          this.x += (targetX7 - this.x) * 0.18;
          this.y += (targetY7 - this.y) * 0.18;

          const loadFrame = Math.floor(elapsed / 125) % 2;
          this.img.src = loadFrame === 0 ? SPRITES.loading : SPRITES.think;
          if (Math.random() < 0.05) {
            spawnParticle(this.x + 30, this.y + 20, 'star');
          }
          break;

        case 8: // Answering
          const targetX8 = Math.max(20, window.innerWidth - 490);
          const targetY8 = window.innerHeight - 95;
          this.x += (targetX8 - this.x) * 0.18;
          this.y += (targetY8 - this.y) * 0.18;

          if (elapsed < 1000) {
            this.img.src = SPRITES.excited;
          } else {
            const ansFrame = Math.floor(elapsed / 150) % 2;
            this.img.src = ansFrame === 0 ? SPRITES.answering : SPRITES.askInvite;
          }
          break;

        case 8.5: // Multi-Question Wait
          const targetX85 = Math.max(20, window.innerWidth - 490);
          const targetY85 = window.innerHeight - 95;
          this.x += (targetX85 - this.x) * 0.18;
          this.y += (targetY85 - this.y) * 0.18;

          const cycle = Math.floor(elapsed / 4000) % 2;
          this.img.src = cycle === 0 ? SPRITES.ponder : SPRITES.think;
          break;

        case 9: // Departure & Disappearing
          if (elapsed < 250) {
            this.img.src = SPRITES.leave01;
          } else {
            this.img.src = SPRITES.leave02;
          }
          this.opacity = Math.max(0, 1.0 - elapsed / 500);
          if (elapsed >= 500) {
            this.enterState(10); // State 10: Instant Recycle
          }
          break;

        case 10: // Instant Recycle
          this.enterState(0);
          break;
      }

      // Apply transformations to DOM
      this.root.style.left = `${this.x}px`;
      this.root.style.top = `${this.y}px`;
      this.root.style.opacity = this.opacity;
      this.img.style.transform = `scale(${this.scale}) scaleX(${this.flip})`;

      requestAnimationFrame(this.loop);
    }
  }

  // Auto-boot singleton companion on DOM load
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
      window.NahidaCompanion = new NahidaCompanion();
    });
  } else {
    window.NahidaCompanion = new NahidaCompanion();
  }
})();
