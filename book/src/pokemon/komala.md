<details class="pokemon-card-container">
<summary>Komala (#181)</summary>
Types: Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Comatose
- Cheek Pouch *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=wish">TM01 - Wish</a>
- <a href="move-lookup.html?q=calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=brick-break">TM31 - Brick Break</a>
- <a href="move-lookup.html?q=double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>

**Encounter Locations**
- Erinys Path (West) — Grass (Day) (10%)
- Wakewater Isle — Grass (Day) (4%)
- Wakewater Isle — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="komala" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-high">115</span> |
| Defense | <span class="stat-value stat-mid">85</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-high">95</span> |
| Speed | <span class="stat-value stat-mid">65</span> |
| Total | <span class="stat-value stat-mid">510</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a> (Lv 1)
- <a href="move-lookup.html?q=rollout">Rollout</a> (Lv 1)
- <a href="move-lookup.html?q=stockpile">Stockpile</a> (Lv 6)
- <a href="move-lookup.html?q=spit-up">Spit Up</a> (Lv 6)
- <a href="move-lookup.html?q=swallow">Swallow</a> (Lv 6)
- <a href="move-lookup.html?q=rapid-spin">Rapid Spin</a> (Lv 11)
- <a href="move-lookup.html?q=yawn">Yawn</a> (Lv 16)
- <a href="move-lookup.html?q=slam">Slam</a> (Lv 21)
- <a href="move-lookup.html?q=bulldoze">Bulldoze</a> (Lv 25)
- <a href="move-lookup.html?q=flail">Flail</a> (Lv 28)
- <a href="move-lookup.html?q=rock-smash">Rock Smash</a> (Lv 30)
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a> (Lv 31)
- <a href="move-lookup.html?q=body-slam">Body Slam</a> (Lv 35)
- <a href="move-lookup.html?q=wood-hammer">Wood Hammer</a> (Lv 38)
- <a href="move-lookup.html?q=psych-up">Psych Up</a> (Lv 43)
- <a href="move-lookup.html?q=play-rough">Play Rough</a> (Lv 45)

**Egg Moves**
- <a href="move-lookup.html?q=charm">Charm</a>
- <a href="move-lookup.html?q=wish">Wish</a>
- <a href="move-lookup.html?q=play-rough">Play Rough</a>
- <a href="move-lookup.html?q=sing">Sing</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=defense-curl">Defense Curl</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=rollout">Rollout</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
