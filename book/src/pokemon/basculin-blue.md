<details class="pokemon-card-container">
<summary>Basculin Blue? (#287)</summary>
Types: Water • Egg Groups: Water 2

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Rock Head
- Adaptability
- Mold Breaker *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Water (0.5×)
- Ice (0.5×)
- Steel (0.5×)

*Weak to*
- Electric (2×)
- Grass (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM12 - Taunt
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM32 - Double Team
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM56 - Scald
- HM01 - Cut
- HM03 - Surf
- HM07 - Waterfall
- HM08 - Dive

**Held Item**
Deep Sea Scale
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="basculin-blue" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">70</span> |
| Attack | <span class="stat-value stat-high">102</span> |
| Defense | <span class="stat-value stat-mid">65</span> |
| Sp. Atk | <span class="stat-value stat-mid">80</span> |
| Sp. Def | <span class="stat-value stat-mid">55</span> |
| Speed | <span class="stat-value stat-high">98</span> |
| Total | <span class="stat-value stat-mid">470</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Tail Whip (Lv 1)
- Water Gun (Lv 1)
- Uproar (Lv 3)
- Headbutt (Lv 5)
- Bite (Lv 7)
- Aqua Jet (Lv 9)
- Chip Away (Lv 11)
- Take Down (Lv 14)
- Crunch (Lv 17)
- Aqua Tail (Lv 20)
- Soak (Lv 23)
- Double-Edge (Lv 26)
- Scary Face (Lv 28)
- Wave Crash (Lv 32)
- Flail (Lv 34)
- Final Gambit (Lv 38)
- Zen Headbutt (Lv 40)
- Thrash (Lv 42)
- Head Smash (Lv 46)

**Egg Moves**
- Swift
- Bubble Beam
- Mud Shot
- Muddy Water
- Agility
- Whirlpool
- Rage
- Brine
- Revenge
- Head Smash

**Tutor Moves**
- Double-Edge
- Endure
- Icy Wind
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
