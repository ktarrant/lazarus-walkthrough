<details class="pokemon-card-container">
<summary>Minior Core (#154)</summary>
Types: Rock / Flying • Egg Groups: Mineral

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Shields Down

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Ground (0×)
- Flying (0.5×)
- Bug (0.5×)

*Weak to*
- Water (2×)
- Electric (2×)
- Ice (2×)
- Rock (2×)
- Steel (2×)

**TM/HM Moves**
- TM04 - Calm Mind
- TM06 - Toxic
- TM16 - Light Screen
- TM17 - Protect
- TM22 - Solar Beam
- TM26 - Earthquake
- TM29 - Psychic
- TM32 - Double Team
- TM33 - Reflect
- TM37 - Sandstorm
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM49 - Bulldoze
- TM54 - Dazzling Gleam

**Held Item**
Star Piece
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="minior-core" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">60</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-high">110</span> |
| Sp. Def | <span class="stat-value stat-mid">65</span> |
| Speed | <span class="stat-value stat-high">120</span> |
| Total | <span class="stat-value stat-mid">530</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Defense Curl (Lv 3)
- Rollout (Lv 8)
- Confuse Ray (Lv 10)
- Gust (Lv 13)
- Swift (Lv 16)
- Ancient Power (Lv 17)
- Self-Destruct (Lv 22)
- Stealth Rock (Lv 24)
- Acrobatics (Lv 27)
- Take Down (Lv 29)
- Autotomize (Lv 31)
- Cosmic Power (Lv 34)
- Power Gem (Lv 37)
- Dazzling Gleam (Lv 40)
- Double-Edge (Lv 43)
- Shell Smash (Lv 45)
- Explosion (Lv 50)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Defense Curl
- Double-Edge
- Endure
- Explosion
- Psych Up
- Rock Slide
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swift
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</details>
