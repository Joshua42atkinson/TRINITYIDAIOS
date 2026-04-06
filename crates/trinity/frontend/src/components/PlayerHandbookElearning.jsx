import React, { useState, useRef, useEffect, useMemo } from 'react';
import { marked } from 'marked';

// ============================================================================
// EDITORIAL STRUCTURE
// ============================================================================
const EDITORIAL_STRUCTURE = [
    { type: 'title', title: "The Player's Handbook", subtitle: 'A Guide to Conscious Learning in Trinity ID AI OS', artIndex: 1 },
    { type: 'toc', title: 'Table of Contents', artIndex: 1 },
    { type: 'content', section: 'Preface', title: 'Preface: A Note on Operating Systems', artIndex: 1, audioFile: '01__Preface_A_Note_on_Operating_Systems.wav' },
    { type: 'part', title: 'Part I', subtitle: 'The Player — Know Thyself', artIndex: 2, audioFile: '02__Part_I_The_Player_Know_Thyself.wav' },
    { type: 'content', section: 'The First Wave', chapterTitle: 'Chapter 1: The Awakening', title: 'The First Wave', artIndex: 3, audioFile: '03__Chapter_1_The_Awakening.wav' },
    { type: 'content', section: 'The Great Fusion', title: 'The Great Fusion', artIndex: 3 },
    { type: 'content', section: 'The Chariot', title: 'The Chariot', artIndex: 3 },
    { type: 'content', section: 'The Pendulum', chapterTitle: 'Chapter 2: The Lens of Belief', title: 'The Pendulum', artIndex: 4, audioFile: '04__Chapter_2_The_Lens_of_Belief.wav' },
    { type: 'content', section: 'The Rendering Engine', title: 'The Rendering Engine', artIndex: 4 },
    { type: 'part', title: 'Part II', subtitle: 'The Character Sheet — Build Thyself', artIndex: 5, audioFile: '05__Part_II_The_Character_Sheet_Build_Thyself.wav' },
    { type: 'content', section: 'The Tyrant in the Kitchen', chapterTitle: 'Chapter 3: The Shape of the Soul', title: 'The Tyrant in the Kitchen', artIndex: 6, audioFile: '06__Chapter_3_The_Shape_of_the_Soul_Attributes_.wav' },
    { type: 'content', section: 'The Imbalanced Wheel', title: 'The Imbalanced Wheel', artIndex: 6 },
    { type: 'content', section: 'The Radar Chart', title: 'The Radar Chart', artIndex: 6 },
    { type: 'content', section: "The Marine's Dilemma", chapterTitle: 'Chapter 4: The Virtue Topology', title: "The Marine's Dilemma", artIndex: 7, audioFile: '07__Chapter_4_The_Virtue_Topology_Alignment_.wav' },
    { type: 'content', section: 'The Three Primary Colors of Motivation', title: 'The Three Primary Colors of Motivation', artIndex: 7 },
    { type: 'part', title: 'Part III', subtitle: 'The Campaign — Level Up', artIndex: 8, audioFile: '08__Part_III_The_Campaign_Level_Up.wav' },
    { type: 'content', section: 'The Glass Cannon', chapterTitle: 'Chapter 5: The Zone of Polarity', title: 'The Glass Cannon', artIndex: 9, audioFile: '09__Chapter_5_The_Zone_of_Polarity.wav' },
    { type: 'content', section: 'The Pendulum Trap', title: 'The Pendulum Trap', artIndex: 9 },
    { type: 'content', section: 'Single-Player Mode', chapterTitle: 'Chapter 6: The Observer', title: 'Single-Player Mode', artIndex: 10, audioFile: '10__Chapter_6_The_Observer.wav' },
    { type: 'content', section: 'Constraint Building', title: 'Constraint Building', artIndex: 10 },
    { type: 'content', section: 'The River', chapterTitle: 'Chapter 7: The Best Self', title: 'The River', artIndex: 11, audioFile: '11__Chapter_7_The_Best_Self.wav' },
    { type: 'content', section: 'The Harmony of Polarity', title: 'The Harmony of Polarity', artIndex: 11 },
    { type: 'part', title: 'Part IV', subtitle: 'The Toolkit — Core Skills', artIndex: 12, audioFile: '12__Part_IV_The_Toolkit_Core_Skills.wav' },
    { type: 'content', section: "The Bartender's Secret", chapterTitle: 'Chapter 8: Stewardship', title: "The Bartender's Secret", artIndex: 13, audioFile: '13__Chapter_8_Stewardship.wav' },
    { type: 'content', section: 'The Internal Economy', title: 'The Internal Economy', artIndex: 13 },
    { type: 'content', section: 'The Exit Interview', chapterTitle: 'Chapter 9: Ownership', title: 'The Exit Interview', artIndex: 14, audioFile: '14__Chapter_9_Ownership_The_Great_Recycler.wav' },
    { type: 'content', section: 'The Great Recycler', title: 'The Great Recycler', artIndex: 14 },
    { type: 'content', section: 'The Cost of Repairs', chapterTitle: 'Chapter 10: Vulnerability', title: 'The Cost of Repairs', artIndex: 15, audioFile: '15__Chapter_10_Vulnerability_The_Antenna.wav' },
    { type: 'content', section: 'The Sensitive Antenna', title: 'The Sensitive Antenna', artIndex: 15 },
    { type: 'content', section: 'The Ghost Train', title: 'The Ghost Train', artIndex: 15 },
    { type: 'part', title: 'Part V', subtitle: 'The Forge — Practice Makes Permanent', artIndex: 16, audioFile: '16__Part_V_The_Forge_Practice_Makes_Permanent.wav' },
    { type: 'content', section: 'The Freezer Door', chapterTitle: 'Chapter 11: The Body as a Forge', title: 'The Freezer Door', artIndex: 17, audioFile: '17__Chapter_11_The_Body_as_a_Forge.wav' },
    { type: 'content', section: 'The Alchemical Engine', title: 'The Alchemical Engine', artIndex: 17 },
    { type: 'content', section: 'The Only Workout That Matters', chapterTitle: 'Chapter 12: Forging the Will', title: 'The Only Workout That Matters', artIndex: 18, audioFile: '18__Chapter_12_Forging_the_Will.wav' },
    { type: 'content', section: 'The Manual Override', title: 'The Manual Override', artIndex: 18 },
    { type: 'part', title: 'Part VI', subtitle: 'Multiplayer — The Social Forge', artIndex: 19, audioFile: '19__Part_VI_Multiplayer_The_Social_Forge.wav' },
    { type: 'content', section: 'The Eye of the Other', chapterTitle: 'Chapter 13: The Mirror Mechanic', title: 'The Eye of the Other', artIndex: 20, audioFile: '20__Chapter_13_The_Mirror_Mechanic.wav' },
    { type: 'content', section: 'The Schema as a Tool', title: 'The Schema as a Tool', artIndex: 20 },
    { type: 'content', section: 'The Circle of Trust', chapterTitle: 'Chapter 14: The University', title: 'The Circle of Trust', artIndex: 21, audioFile: '21__Chapter_14_The_University.wav' },
    { type: 'content', section: 'The Vampire vs. The Architect', title: 'The Vampire vs. The Architect', artIndex: 21 },
    { type: 'content', section: 'Choosing Your Build', chapterTitle: 'Chapter 15: Prestige Classes', title: 'Choosing Your Build', artIndex: 22, audioFile: '22__Chapter_15_Prestige_Classes.wav' },
    { type: 'content', section: 'The Locomotive Profiles', title: 'The Locomotive Profiles', artIndex: 22 },
    { type: 'content', section: 'Epilogue', title: 'Epilogue: The Worth-It Protocol', artIndex: 23, audioFile: '23__Epilogue_The_Worth-It_Protocol.wav' },
    { type: 'finale', section: 'About the Author', title: 'About the Author', artIndex: 24, audioFile: '24__About_the_Author.wav' }
];

function splitIntoPages(markdown, maxChars = 900) {
    if (!markdown || markdown.trim().length === 0) return [''];
    const rawBlocks = markdown.split(/\n\n+/).filter(b => b.trim().length > 0);
    const pages = [];
    let currentBlocks = [];
    let currentLen = 0;

    for (let i = 0; i < rawBlocks.length; i++) {
        const block = rawBlocks[i].trim();
        const isTrinityBox = block.includes('IN TRINITY');
        const isTable = block.includes('|') && block.includes('---');
        if (isTrinityBox || isTable) {
            if (currentBlocks.length > 0) {
                pages.push(currentBlocks.join('\n\n'));
                currentBlocks = [];
                currentLen = 0;
            }
            pages.push(block);
            continue;
        }
        if (currentLen + block.length > maxChars && currentBlocks.length > 0) {
            pages.push(currentBlocks.join('\n\n'));
            currentBlocks = [block];
            currentLen = block.length;
        } else {
            currentBlocks.push(block);
            currentLen += block.length;
        }
    }
    if (currentBlocks.length > 0) pages.push(currentBlocks.join('\n\n'));
    return pages.length > 0 ? pages : [''];
}

export default function PlayerHandbookElearning({ onBack }) {
    const [spreadIndex, setSpreadIndex] = useState(0);
    const [isPlaying, setIsPlaying] = useState(false);
    const [playbackRate, setPlaybackRate] = useState(1.0);
    const [flipAnime, setFlipAnime] = useState('');
    const [sectionMap, setSectionMap] = useState({});

    const audioRef = useRef(null);
    const flipTimer = useRef(null);

    useEffect(() => {
        fetch('/docs/PLAYERS_HANDBOOK.md')
            .then(res => res.text())
            .then(text => {
                const map = {};
                const chunks = text.split(/(?=^### )/m);
                for (const chunk of chunks) {
                    const firstLine = chunk.split('\n')[0].trim();
                    if (firstLine.startsWith('### ')) {
                        const title = firstLine.replace(/^###\s+/, '');
                        map[title] = chunk.split('\n').slice(1).join('\n');
                    }
                }
                const chapterChunks = text.split(/(?=^## )/m);
                for (const chunk of chapterChunks) {
                    const firstLine = chunk.split('\n')[0].trim();
                    if (firstLine.startsWith('## Preface')) {
                        const lines = chunk.split('\n').slice(1);
                        const endIdx = lines.findIndex(l => l.startsWith('### '));
                        map['Preface'] = (endIdx >= 0 ? lines.slice(0, endIdx) : lines).join('\n');
                    }
                    if (firstLine.startsWith('## Epilogue')) {
                        map['Epilogue'] = chunk.split('\n').slice(1).join('\n');
                    }
                    if (firstLine.startsWith('## About')) {
                        map['About the Author'] = chunk.split('\n').slice(1).join('\n');
                    }
                }
                const tocMatch = text.match(/## Table of Contents\n([\s\S]*?)(?=\n---)/);
                if (tocMatch) map['__toc__'] = tocMatch[1];
                setSectionMap(map);
            })
            .catch(err => console.error(err));
    }, []);

    const spreads = useMemo(() => {
        const pages = [];
        for (const entry of EDITORIAL_STRUCTURE) {
            if (entry.type === 'content' || entry.type === 'finale') {
                const md = sectionMap[entry.section] || '';
                const subPages = splitIntoPages(md);
                subPages.forEach((content, i) => {
                    pages.push({
                        ...entry,
                        type: entry.type === 'finale' && i === subPages.length - 1 ? 'finale' : 'content',
                        content,
                        displayTitle: i === 0 ? (entry.chapterTitle ? `${entry.chapterTitle} — ${entry.title}` : entry.title) : entry.title,
                        isFirstSubpage: i === 0,
                        isContinuation: i > 0,
                        subPageIndex: i,
                        subPageTotal: subPages.length
                    });
                });
            } else {
                pages.push({ ...entry, displayTitle: entry.title, isFirstSubpage: true });
            }
        }

        const spreadArray = [];
        let i = 0;
        let globalPageIdx = 1;
        while (i < pages.length) {
            const page = pages[i];
            
            // If it demands an art break (first page of chapter/part)
            if ((page.type === 'part' || page.type === 'title' || page.type === 'toc' || page.type === 'finale' || (page.type === 'content' && page.isFirstSubpage)) && page.artIndex) {
                 const rightPage = { ...page, globalPage: globalPageIdx + 1 };
                 spreadArray.push([
                     { isArtOnly: true, splashUrl: `/audiobook_art/chapter_${page.artIndex}.jpg`, globalPage: globalPageIdx },
                     rightPage
                 ]);
                 i++;
                 globalPageIdx += 2;
            } else {
                 const leftPage = { ...page, globalPage: globalPageIdx };
                 let rightPage = null;
                 
                 // Can we safely place the next page on the right?
                 if (i + 1 < pages.length) {
                     const nextParam = pages[i+1];
                     // If the next page demands art on the left, we cannot put it on the right
                     if ((nextParam.type === 'part' || nextParam.type === 'title' || nextParam.type === 'toc' || nextParam.type === 'finale' || (nextParam.type === 'content' && nextParam.isFirstSubpage)) && nextParam.artIndex) {
                         // Next page starts a new spread art, so right page is null
                     } else {
                         rightPage = { ...nextParam, globalPage: globalPageIdx + 1 };
                     }
                 }
                 
                 spreadArray.push([ leftPage, rightPage ]);
                 i += rightPage ? 2 : 1;
                 globalPageIdx += 2;
            }
        }
        return spreadArray;
    }, [sectionMap]);

    const currentSpread = spreads[spreadIndex] || [null, null];
    const leftPage = currentSpread[0];
    const rightPage = currentSpread[1];
    
    // Find audio from whichever page has it
    const audioPage = (leftPage && leftPage.audioFile && leftPage.isFirstSubpage) ? leftPage : 
                      (rightPage && rightPage.audioFile && rightPage.isFirstSubpage) ? rightPage : null;
    const audioUrl = audioPage ? `/audiobook/${audioPage.audioFile}` : null;

    useEffect(() => { if (audioRef.current) audioRef.current.playbackRate = playbackRate; }, [playbackRate]);
    useEffect(() => {
        if (isPlaying && audioRef.current && audioUrl) {
            audioRef.current.play().catch(() => setIsPlaying(false));
        }
    }, [spreadIndex]);

    const handlePlayPause = () => {
        if (!audioRef.current || !audioUrl) return;
        if (isPlaying) audioRef.current.pause(); else audioRef.current.play();
        setIsPlaying(!isPlaying);
    };

    const handleFlip = (direction, targetIdx) => {
        setFlipAnime(direction);
        if (flipTimer.current) clearTimeout(flipTimer.current);
        flipTimer.current = setTimeout(() => setFlipAnime(''), 600);
        setTimeout(() => setSpreadIndex(targetIdx), 300);
    };

    const nextSlide = () => { if (spreadIndex < spreads.length - 1) handleFlip('flip-forward', spreadIndex + 1); };
    const prevSlide = () => { if (spreadIndex > 0) handleFlip('flip-backward', spreadIndex - 1); };

    const getHtml = (page) => {
        if (!page) return '';
        if (page.content) return marked(page.content);
        if (page.type === 'toc') return marked(sectionMap['__toc__'] || '*Loading...*');
        return '';
    };

    const renderPageContent = (page) => {
        if (!page || page.isArtOnly) return null;
        const currentHTML = getHtml(page);
        const spotUrl = page.artIndex ? `/audiobook_art/chapter_${page.artIndex}_spot.jpg` : null;
        const hasTrinityBox = page.content && page.content.includes('IN TRINITY');

        switch (page.type) {
            case 'title':
                return (
                    <div style={{ textAlign: 'center', margin: 'auto', maxWidth: '500px' }}>
                        <div className="gold-rule" />
                        <h1 className="book-title">{page.title}</h1>
                        <p className="book-subtitle">{page.subtitle}</p>
                        <div className="gold-rule" />
                        <p style={{ color: '#8b7355', fontSize: '14px', marginTop: '40px', fontFamily: "'Inter', sans-serif" }}>
                            Version 1.0 · March 2026<br/>Joshua Atkinson · Purdue University
                        </p>
                        <div style={{ marginTop: '40px', fontSize: '12px', color: '#a0876e', fontFamily: "'Inter', sans-serif" }}>
                            License: Apache 2.0
                        </div>
                    </div>
                );
            case 'part':
                return (
                    <div style={{ textAlign: 'center', margin: 'auto', maxWidth: '500px' }}>
                        <img src={spotUrl} className="part-art" alt="Part Art" onError={e => { e.target.style.display = 'none'; }} />
                        <div className="gold-rule" style={{ marginTop: '32px' }}/>
                        <h1 className="part-heading">{page.title}</h1>
                        <p className="part-subtitle">{page.subtitle}</p>
                        <div className="gold-rule" />
                    </div>
                );
            case 'toc':
                return (
                    <>
                        <div className="page-header">
                            <h1 className="section-title">Table of Contents</h1>
                        </div>
                        <div className="dnd-markdown toc-page" dangerouslySetInnerHTML={{ __html: currentHTML }} />
                    </>
                );
            case 'finale':
                return (
                    <div style={{ textAlign: 'center', margin: 'auto', maxWidth: '500px' }}>
                        <img src={spotUrl} className="part-art" alt="Author" style={{ width: '200px', height: '200px' }} onError={e => { e.target.style.display = 'none'; }} />
                        <h1 className="part-heading" style={{ fontSize: '36px', marginTop: '24px' }}>{page.displayTitle}</h1>
                        <div className="gold-rule" />
                        <div className="dnd-markdown" style={{ textAlign: 'left' }} dangerouslySetInnerHTML={{ __html: currentHTML }} />
                    </div>
                );
            default: // content
                return (
                    <>
                        {page.isFirstSubpage && (
                            <div className="page-header">
                                {page.chapterTitle && <div className="chapter-label">{page.chapterTitle}</div>}
                                <h1 className="section-title">{page.isContinuation ? page.title : page.displayTitle}</h1>
                            </div>
                        )}
                        {page.isContinuation && (
                            <div className="continuation-label">{page.title} <span style={{opacity:0.5}}>— continued</span></div>
                        )}
                        <div className="dnd-markdown" style={{ flex: 1 }}>
                            {page.isFirstSubpage && !hasTrinityBox && spotUrl && (
                                <img src={spotUrl} alt="" className="spot-art" onError={e => { e.target.style.display = 'none'; }} />
                            )}
                            <div dangerouslySetInnerHTML={{ __html: currentHTML }} />
                        </div>
                    </>
                );
        }
    };

    return (
        <div className="handbook-container">
            <button onClick={onBack} className="stow-btn">← Stow Handbook</button>
            <div className="page-counter">Spread {spreadIndex + 1} of {spreads.length}</div>

            <div className={`book-spread ${flipAnime}`}>
                <div className="book-spine" />

                {/* Left Page */}
                <div className={`book-page left-page ${leftPage && leftPage.isArtOnly ? 'art-bg' : 'content-bg'}`}>
                    {leftPage && leftPage.isArtOnly && (
                        <div className="art-frame">
                            <img src={leftPage.splashUrl} alt="" className="splash-img" onError={e => { e.target.parentElement.style.backgroundColor = '#111'; }} />
                        </div>
                    )}
                    {leftPage && !leftPage.isArtOnly && renderPageContent(leftPage)}
                    {leftPage && !leftPage.isArtOnly && <div className="page-watermark-left">{leftPage.globalPage}</div>}
                </div>

                {/* Right Page */}
                <div className={`book-page right-page ${rightPage ? 'content-bg' : 'art-bg'}`}>
                    {rightPage && renderPageContent(rightPage)}
                    {rightPage && <div className="page-watermark-right">{rightPage.globalPage}</div>}
                </div>
            </div>

            <div className="audio-bar">
                {audioUrl && (
                    <audio ref={audioRef} src={audioUrl} preload="auto" onEnded={() => {
                        setIsPlaying(false);
                        if (spreadIndex < spreads.length - 1) nextSlide();
                    }} />
                )}
                <div className="nav-group">
                    <button onClick={prevSlide} disabled={spreadIndex === 0}
                        className={`nav-btn ${spreadIndex === 0 ? 'nav-btn--disabled' : ''}`}>
                        ◀ Turn Back
                    </button>
                    <button onClick={handlePlayPause} className={`play-btn ${audioUrl ? '' : 'play-btn--disabled'}`}>
                        {!audioUrl ? 'NO NARRATION' : isPlaying ? 'PAUSE NARRATION' : 'READ TOME'}
                    </button>
                    <button onClick={nextSlide} disabled={spreadIndex === spreads.length - 1}
                        className={`nav-btn ${spreadIndex === spreads.length - 1 ? 'nav-btn--disabled' : ''}`}>
                        Turn Page ▶
                    </button>
                </div>
                <div className="speed-group">
                    {[0.75, 1.0, 1.25, 1.5].map(rate => (
                        <button key={rate} onClick={() => { setPlaybackRate(rate); if (audioRef.current) audioRef.current.playbackRate = rate; }}
                            className={`speed-btn ${playbackRate === rate ? 'speed-btn--active' : ''}`}>
                            {rate}x
                        </button>
                    ))}
                </div>
            </div>

            <style dangerouslySetInnerHTML={{__html: `
                @import url('https://fonts.googleapis.com/css2?family=Cinzel:wght@400;700;900&family=Inter:wght@400;500;600&display=swap');

                .handbook-container {
                    grid-column: 1 / -1; grid-row: 2;
                    display: flex; flex-direction: column;
                    align-items: center; justify-content: center;
                    background: #0a0a0a;
                    height: 100%; width: 100%; overflow: hidden;
                    padding: 16px; position: relative;
                }

                .stow-btn {
                    position: absolute; top: 16px; left: 32px; z-index: 50;
                    padding: 8px 16px; background: rgba(15,12,8,0.9);
                    border: 1px solid #CFB991; color: #CFB991;
                    font-family: 'Cinzel', serif; font-size: 13px; cursor: pointer;
                    border-radius: 4px; transition: all 0.2s;
                }
                .stow-btn:hover { background: rgba(207,185,145,0.15); }
                .page-counter {
                    position: absolute; top: 16px; right: 32px; z-index: 50;
                    color: #CFB991; font-family: 'Inter', sans-serif; font-size: 13px;
                    background: rgba(15,12,8,0.9); padding: 6px 16px;
                    border-radius: 4px; border: 1px solid rgba(207,185,145,0.3);
                }

                .book-spread {
                    display: flex; width: 100%; max-width: 1400px; height: calc(100% - 80px);
                    background: #2a2015; /* Base for empty pages */
                    box-shadow: 0 30px 60px rgba(0,0,0,0.8), inset 0 0 80px rgba(139,115,85,0.2);
                    border-radius: 4px 8px 8px 4px;
                    border: 2px solid #5a4b3c;
                    position: relative; overflow: hidden;
                    transition: opacity 0.6s ease, transform 0.6s ease;
                }
                .flip-forward { opacity: 0; transform: translateX(80px); }
                .flip-backward { opacity: 0; transform: translateX(-80px); }

                .book-spine {
                    position: absolute; top: 0; bottom: 0; left: 50%; width: 40px; margin-left: -20px;
                    background: linear-gradient(to right, rgba(0,0,0,0.4), rgba(0,0,0,0.8) 50%, rgba(0,0,0,0.4));
                    z-index: 10; pointer-events: none;
                }

                .book-page {
                    flex: 1; position: relative; z-index: 5;
                    overflow: hidden; display: flex; flex-direction: column;
                }
                
                .left-page {
                    padding: 36px 52px 28px 44px; /* Inner padding differs from right */
                    box-shadow: inset -25px 0 25px rgba(0,0,0,0.15);
                }
                
                .right-page {
                    padding: 36px 44px 28px 52px;
                    box-shadow: inset 25px 0 25px rgba(0,0,0,0.15);
                }

                .art-bg {
                    background: #1a1510;
                    padding: 24px;
                }
                
                .content-bg {
                    background: linear-gradient(135deg, #f4ebd8 0%, #ede2cc 60%, #e8d9bf 100%);
                }

                .art-frame {
                    width: 100%; height: 100%;
                    border: 10px solid #2a2015;
                    outline: 2px solid #CFB991; outline-offset: -5px;
                    overflow: hidden; position: relative;
                    box-shadow: inset 0 0 40px rgba(0,0,0,0.6);
                    background: #111;
                }
                .splash-img {
                    width: 100%; height: 100%; object-fit: cover;
                }

                .page-watermark-left {
                    position: absolute; bottom: 12px; left: 20px;
                    font-family: 'Cinzel', serif; font-size: 14px; color: rgba(139,115,85,0.4);
                }
                .page-watermark-right {
                    position: absolute; bottom: 12px; right: 20px;
                    font-family: 'Cinzel', serif; font-size: 14px; color: rgba(139,115,85,0.4);
                }

                .gold-rule {
                    height: 3px; width: 40%; margin: 24px auto;
                    background: linear-gradient(to right, transparent, #CFB991, transparent);
                }
                .book-title {
                    color: #5A1B1B; font-size: 46px; line-height: 1.05;
                    font-family: 'Cinzel', serif; font-weight: 900;
                    letter-spacing: 2px; text-shadow: 1px 2px 0px rgba(255,255,255,0.6);
                    margin: 0;
                }
                .book-subtitle {
                    color: #5a4b3c; font-size: 17px; font-style: italic;
                    font-family: 'Inter', sans-serif; margin-top: 16px;
                }
                .part-art {
                    width: 160px; height: 160px; border-radius: 50%;
                    border: 4px solid #CFB991; object-fit: cover;
                    box-shadow: 0 10px 30px rgba(0,0,0,0.3);
                }
                .part-heading {
                    color: #5A1B1B; font-size: 44px; font-family: 'Cinzel', serif;
                    font-weight: 900; letter-spacing: 3px;
                    text-shadow: 1px 2px 0px rgba(255,255,255,0.5);
                    margin: 0;
                }
                .part-subtitle {
                    color: #2a2015; font-size: 22px; font-family: 'Cinzel', serif;
                    margin-top: 8px; font-weight: 700;
                }

                .page-header {
                    border-bottom: 2px solid #CFB991;
                    padding-bottom: 8px; margin-bottom: 16px;
                }
                .chapter-label {
                    font-family: 'Inter', sans-serif; font-size: 11px;
                    text-transform: uppercase; letter-spacing: 3px;
                    color: #8b7355; margin-bottom: 4px;
                }
                .section-title {
                    color: #2a2015; font-size: 26px; line-height: 1.15;
                    font-weight: 900; margin: 0; font-family: 'Cinzel', serif;
                    text-shadow: 0.5px 0.5px 0px rgba(255,255,255,0.7);
                }
                .continuation-label {
                    font-family: 'Cinzel', serif; font-size: 18px;
                    color: #5A1B1B; margin-bottom: 12px;
                    border-bottom: 1px solid rgba(207,185,145,0.4);
                    padding-bottom: 8px;
                }

                .spot-art {
                    float: right; width: 140px; height: 140px;
                    margin: 0 0 16px 16px; border-radius: 50%;
                    border: 3px solid #8b7355;
                    box-shadow: 0 6px 12px rgba(0,0,0,0.15);
                    shape-outside: circle(50%); object-fit: cover;
                }

                .dnd-markdown {
                    font-family: 'Inter', sans-serif;
                    color: #2a2015; font-size: 14px; line-height: 1.6;
                }
                .dnd-markdown > p:first-of-type::first-letter {
                    font-family: 'Cinzel', serif; font-size: 3.2em;
                    float: left; margin: 6px 6px 0 -2px;
                    line-height: 0.8; color: #5A1B1B;
                    text-shadow: 1px 1px 2px rgba(0,0,0,0.15);
                }
                .dnd-markdown p { margin-bottom: 12px; text-align: justify; }
                .dnd-markdown h1, .dnd-markdown h2, .dnd-markdown h3 {
                    font-family: 'Cinzel', serif; color: #5A1B1B;
                    margin-top: 16px; margin-bottom: 8px;
                    border-bottom: 1px solid rgba(207,185,145,0.4);
                }

                .dnd-markdown blockquote {
                    background: linear-gradient(135deg, rgba(207,185,145,0.12), rgba(207,185,145,0.25));
                    border: 2px solid #CFB991; border-radius: 4px;
                    padding: 14px 16px; margin: 12px 0;
                    box-shadow: inset 0 0 15px rgba(0,0,0,0.04), 0 3px 8px rgba(0,0,0,0.08);
                    position: relative;
                }
                .dnd-markdown blockquote::before {
                    content: ''; position: absolute; top: -3px; left: -3px;
                    width: 14px; height: 14px;
                    border-top: 2px solid #5A1B1B; border-left: 2px solid #5A1B1B;
                }
                .dnd-markdown blockquote::after {
                    content: ''; position: absolute; bottom: -3px; right: -3px;
                    width: 14px; height: 14px;
                    border-bottom: 2px solid #5A1B1B; border-right: 2px solid #5A1B1B;
                }
                .dnd-markdown blockquote p {
                    margin-bottom: 0; font-style: italic; font-size: 13px;
                }
                .dnd-markdown ul, .dnd-markdown ol { margin-bottom: 12px; padding-left: 20px; }
                .dnd-markdown li { margin-bottom: 4px; }
                .dnd-markdown strong { color: #111; }
                .dnd-markdown em { color: #5A1B1B; }

                .dnd-markdown table {
                    width: 100%; border-collapse: collapse; margin: 8px 0;
                    font-size: 13px;
                }
                .dnd-markdown th, .dnd-markdown td {
                    border: 1px solid #8b7355; padding: 8px 10px; text-align: left;
                }
                .dnd-markdown th {
                    background: rgba(90,27,27,0.08);
                    font-family: 'Cinzel', serif; font-size: 12px;
                    color: #5A1B1B; font-weight: 700;
                }
                .dnd-markdown td { background: rgba(255,255,255,0.3); }

                .toc-page { font-size: 13px; }
                .toc-page ul { list-style: none; padding: 0; }
                .toc-page li { padding: 3px 0; border-bottom: 1px dotted rgba(139,115,85,0.3); }
                .toc-page a { color: #5A1B1B; text-decoration: none; }
                .toc-page li li { padding-left: 20px; border-bottom: none; font-size: 12px; }

                .audio-bar {
                    margin-top: 12px; padding: 10px 24px;
                    width: 100%; max-width: 1400px;
                    background: rgba(15,12,8,0.9);
                    border: 1px solid rgba(207,185,145,0.4);
                    border-radius: 6px;
                    display: flex; align-items: center; justify-content: space-between;
                    box-shadow: 0 8px 16px rgba(0,0,0,0.4); z-index: 50;
                }
                .nav-group { display: flex; gap: 16px; align-items: center; }
                .nav-btn {
                    background: transparent; border: none;
                    color: #CFB991; font-family: 'Cinzel', serif;
                    font-weight: 700; font-size: 13px;
                    cursor: pointer; transition: all 0.2s; outline: none;
                }
                .nav-btn:hover:not(.nav-btn--disabled) { color: #fff; }
                .nav-btn--disabled { opacity: 0.3; cursor: default; }
                .play-btn {
                    font-size: 14px; padding: 8px 24px;
                    background: linear-gradient(to bottom, #4a3a2a, #2a2015);
                    color: #f4ebd8; border-radius: 4px;
                    border: 1px solid #CFB991;
                    box-shadow: 0 4px 6px rgba(0,0,0,0.3);
                    font-family: 'Cinzel', serif; font-weight: 700;
                    cursor: pointer; transition: all 0.2s;
                }
                .play-btn:hover { background: linear-gradient(to bottom, #5a4a3a, #3a3025); }
                .play-btn--disabled {
                    background: rgba(40,30,20,0.4); color: #8b7355;
                    cursor: default; border-color: rgba(207,185,145,0.3);
                }
                .speed-group {
                    display: flex; gap: 6px;
                    border-left: 1px solid rgba(139,115,85,0.4);
                    padding-left: 16px;
                }
                .speed-btn {
                    background: transparent; border: 1px solid rgba(139,115,85,0.5);
                    color: #CFB991; padding: 4px 10px; font-size: 12px;
                    cursor: pointer; font-family: 'Inter', sans-serif;
                    border-radius: 3px; outline: none; transition: all 0.2s;
                }
                .speed-btn--active {
                    background: #CFB991; color: #111; border-color: #CFB991;
                }
            `}} />
        </div>
    );
}

